use crate::std::prelude::*;

pub fn box_vs_rc() {
    let mut x = Box::new("Box is for single ownership".to_string());

    let mut y = Rc::new("Rc provides shared ownership".to_string());

    println!("\nValue in x is {}", &x);

    println!("\nValue in y is {}", &y);

    //illustrate shared ownership in Rc
    let y1 = y.clone(); // Creates a new reference to the same value. No memory duplication involved
    let y2 = y.clone(); // Creates a new reference to the same value. No memory duplication involved
    //At this point, there are 3 pointers that share the same heap location (y, y1 and y2)
    println!("Reference count for y: {}", Rc::strong_count(&y)); // Output: Reference count: 3.
    println!("Value from y: {}", y);
    // Access the shared value through the references
    println!("Value from y1: {}", y1);
    println!("Value from y2: {}", y2);

    //Box clone behaves differently, like the regular clone that involves deep copy
    let x1 = x.clone(); //Allocates a new Box in heap and deep copies the value. Memory duplication
    let x2 = x.clone(); //Allocates another new Box in heap and deep copies the value. Memory duplication

    println!("\nValue in x is now {}", &x);
    println!("\nValue in y is still {}", &y);

    //No problem with creating a new y
    y = Rc::new("This is a new Rc".to_string());
    println!("Reference count for y is now: {}. What about y1 and y2? They are still references to the previous heap which will remain until all references are dropped", Rc::strong_count(&y)); // Output: Reference count: 1.
    println!("Value from y: {}", y);
    // Access the shared value through the references has not changed for y1 and y1.
    println!("Value from y1: {}", y1);
    println!("Value from y2: {}", y2);

    println!(
        "Reference count for y1 is {}, same as for y2: {}.",
        Rc::strong_count(&y1),
        Rc::strong_count(&y2)
    ); // Output: Reference count: 2.
       //drop y1 and y2
    drop(y1);
    drop(y2);
    //println!("Reference count for y1 is {}, same as for y2: {}." , Rc::strong_count(&y1), Rc::strong_count(&y2)); // y1 and y2 are not available

    //Value in Box can be mutated but not value in Rc
    *x = "Box can be mutated".to_string();
    //*y = "Rc cannot be mutated. So this will not work".to_string();
    //If internal mutation of Rc is required for any element within, we need to
    //wrap that element with RefCell or Cell
}

//RefCell illustration
//RefCell is for interior mutability even when there are immutable reference to the data as is the case of Rc
//RefCell is similar to Cell except that the latter is more flexible and more risky.
//Cell does not track references and more risky. Use it only when
//You need simple and fast interior mutability for single-threaded scenarios and Runtime checks and borrowing rules are unnecessary or undesirable.
pub struct Node {
    value: i32,
    parent: Option<Weak<RefCell<Node>>>, // Use Weak for parent link
    children: Vec<Rc<RefCell<Node>>>,
}

pub fn create_tree() -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node {
        value: 42,
        parent: None,
        children: vec![],
    }));
    add_child(&root);
    root
}

pub fn add_child(tree: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let child = Rc::new(RefCell::new(Node {
        value: 15,
        parent: Some(Rc::downgrade(&tree)), // Use Weak
        children: vec![],
    }));

    tree.borrow_mut().children.push(child.clone()); //thanks to RefCell, we can mutably borrow from Rc. .borrow is also available
    tree.clone() //this is an Rc clone() which does not do deep copy. Just an additional reference pointer in stack to the same location
}

pub fn print_tree(tree: Rc<RefCell<Node>>) {
    println!("Node value: {}", tree.borrow().value);

    for child in &tree.borrow().children {
        print_tree(child.clone());
    }
}

//Assignment 2B
//1. Rewrite add_child() to allow passing of new child Node properties rather than hardcoding it


//A quick look at Arc and Mutex where there are thread safety concerns
//See task_example.rs


