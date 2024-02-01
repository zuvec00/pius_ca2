mod constants;

use core::{fmt, ptr};

use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use constants::font_constants;
use constants::font_constants::{BACKSPACE, BACKUP_CHAR, CHAR_RASTER_HEIGHT, FONT_WEIGHT};
use noto_sans_mono_bitmap::{get_raster, RasterizedChar};

/// Additional vertical space between lines
const LINE_SPACING: usize = 2;

/// Additional horizontal space between characters.
const LETTER_SPACING: usize = 0;

/// Padding from the border. Prevent that font is too close to border.
const BORDER_PADDING: usize = 2;

/// Returns the raster of the given char or the raster of [font_constants::BACKUP_CHAR].
fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}

/// Allows logging text to a pixel-based framebuffer.
pub struct FrameBufferWriter {
    framebuffer: Option<&'static mut [u8]>,
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
}

impl FrameBufferWriter {
    /// Creates a new logger that uses the given framebuffer. Not used anymore as I now lazy statically initialize with empty
    /*pub fn new(framebuffer: Option<&'static mut [u8]>, info: FrameBufferInfo) -> Self {
        let mut logger = Self {
            framebuffer,
            info,
            x_pos: 0,
            y_pos: 0,
        };
        logger.clear();
        logger
    }*/

    ///added for lazy load of Static FrameBufferWriter that will be available with print! everywhere
    pub fn empty() -> Self {
        let logger = Self {
            framebuffer: None,
            x_pos: 0,
            y_pos: 0,
            info: FrameBufferInfo {
                // The total size in bytes.
                byte_len: 0,
                // The width in pixels.
                width: 0,
                // The height in pixels.
                height: 0,
                // The color format of each pixel.
                // The number of bytes per pixel.
                bytes_per_pixel: 0,
                // Number of pixels between the start of a line and the start of the next.
                // Some framebuffers use additional padding at the end of a line, so this
                // value might be larger than horizontal_resolution. It is
                // therefore recommended to use this field for calculating the start address of a line.
                stride: 0,
                pixel_format: PixelFormat::Rgb,
            },
        };
        //logger.clear(); //Should not call clear when the framebuffer is still None
        logger
    }

    ///the static FrameBufferWriter is initialized in my_entry_point by calling this
    pub fn init(&mut self, framebuffer: &'static mut [u8], info: FrameBufferInfo) {
        self.framebuffer = Some(framebuffer);
        self.info = info;
        self.x_pos = 0;
        self.y_pos = 0;

        self.clear();
    }

    //Make it possible to set x, y positions with option to provide both or just one of them
    pub fn set_x_y_pos(&mut self, x_pos: Option<usize>, y_pos: Option<usize>){
        self.x_pos = x_pos.unwrap_or(self.x_pos);
        self.y_pos = y_pos.unwrap_or(self.y_pos);
    }

    fn newline(&mut self) {
        self.y_pos += font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return()
    }

    fn carriage_return(&mut self) {
        self.x_pos = BORDER_PADDING;
    }

    /// Erases all text on the screen. Resets self.x_pos and self.y_pos.
    pub fn clear(&mut self) {
        self.x_pos = BORDER_PADDING;
        self.y_pos = BORDER_PADDING;
        self.framebuffer.as_mut().unwrap().fill(0);
    }

    fn width(&self) -> usize {
        self.info.width
    }

    fn height(&self) -> usize {
        self.info.height
    }

    fn backspace(&mut self) {
        let new_xpos = self.x_pos - font_constants::CHAR_RASTER_WIDTH;

        if new_xpos <= BORDER_PADDING {
            //left border
            //first clear position before moving up in y axis
            self.x_pos = new_xpos;
            self.write_rendered_char(get_char_raster(' '));
            //Move y up if not first line
            if self.y_pos > font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING + BORDER_PADDING
            {
                //not first line
                self.y_pos -= font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
            }
            //move x to the right end
            self.x_pos = self.width() - font_constants::CHAR_RASTER_WIDTH - BORDER_PADDING;
        } else {
            //simply move x one step to the left
            self.x_pos = self.x_pos - font_constants::CHAR_RASTER_WIDTH;
        }

        //clear
        self.write_rendered_char(get_char_raster(' '));
        self.x_pos = self.x_pos - font_constants::CHAR_RASTER_WIDTH;
    }

    /// Writes a single char to the framebuffer. Takes care of special control characters, such as
    /// newlines and carriage returns.
    fn write_char(&mut self, c: char) {
        match c {
            BACKSPACE => self.backspace(),
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                let new_xpos = self.x_pos + font_constants::CHAR_RASTER_WIDTH;
                if new_xpos >= self.width() {
                    self.newline();
                }
                let new_ypos =
                    self.y_pos + font_constants::CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
                if new_ypos >= self.height() {
                    self.clear();
                }
                self.write_rendered_char(get_char_raster(c));
            }
        }
    }

    /// Prints a rendered char into the framebuffer.
    /// Updates self.x_pos.
    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_pos + x, self.y_pos + y, *byte);
            }
        }
        self.x_pos += rendered_char.width() + LETTER_SPACING;
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [intensity, intensity, intensity / 2, 0],
            PixelFormat::Bgr => [intensity / 2, intensity, intensity, 0],
            PixelFormat::U8 => [if intensity > 200 { 0xf } else { 0 }, 0, 0, 0],
            other => {
                // set a supported (but invalid) pixel format before panicking to avoid a double
                // panic; it might not be readable though
                self.info.pixel_format = PixelFormat::Rgb;
                panic!("pixel format {:?} not supported in logger", other)
            }
        };
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer.as_mut().unwrap()[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe { ptr::read_volatile(&self.framebuffer.as_ref().unwrap()[byte_offset]) };
    }
}

unsafe impl Send for FrameBufferWriter {}
unsafe impl Sync for FrameBufferWriter {}

impl fmt::Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}
