use alloc::string::{String, ToString};

use crate::interrupts::KEY_PRESSED;

pub(crate) mod prelude;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        write!($crate::FRAME_BUFFER_WRITER.lock(), "{}", format_args!($($arg)*)).unwrap();
    }};
}

#[macro_export]
#[allow_internal_unstable(print_internals, format_args_nl)]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        write!($crate::FRAME_BUFFER_WRITER.lock(), "{}", format_args_nl!($($arg)*)).unwrap();
    }};
}

#[macro_export]
#[allow_internal_unstable(print_internals, format_args_nl)]
macro_rules! input_str {

    ($prompt:expr) => {{
        print!("{}",$prompt);
        match input_str() {
            Some(value) => value,
            None => "".to_owned(),
        }
    }
        
    };
}

pub fn input_str() -> Option<String> {
    let mut input: String = "".to_string();
    let mut input_counter:u32 = 0; //keep a count so that backspaced induced pop is not allowed beyond the count
    let mut character = *KEY_PRESSED.lock();

    while character != Some('\u{000D}') && character != Some('\u{000A}'){//Test for all three breakout conditions
        match character {
            None => {
                //do nothing
            },
            Some ('\u{0008}') => {//backspace pressed
                *KEY_PRESSED.lock() = None; //clear global KEY_PRESSED so that backspace effect is not repeated
                if input_counter > 0 {
                    print!("{}", character.unwrap());//visually move backwards
                    input.pop(); //pop from input
                    input_counter -=1;
                }
                
            },
            Some ('\u{000A}') => { //escape pressed. Return None from the function immediately
                *KEY_PRESSED.lock() = None; //clear global KEY_PRESSED so that effect is not repeated
                return None;
            },
            Some('\u{000D}') => {//Simply breakout of loop if carriage return is pressed.
                *KEY_PRESSED.lock() = None; //clear global KEY_PRESSED so that effect is not repeated
                break;
            },
            _ => {//Every other unicode key sent, push to input
                let char_received = character.unwrap().clone();//clone it for keep
                print!("{}", &char_received);//show char received on console
                input.push(char_received); //move the character to input
                input_counter+=1; //keep a count so that backspaced induced pop is not allowed beyond the count
                *KEY_PRESSED.lock() = None; //clear global KEY_PRESSED after cloning
            }
        }
        character = *KEY_PRESSED.lock(); //read again as long as we have not broken out.
    };
    Some(input) //return the final input string
}