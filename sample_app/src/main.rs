use std::{cell::RefCell, rc::Rc};

fn main() {
    // Rc pointer lets multiple immutable ownership
    // While RefCell pointer at the same time allows us to mutate the internal value.
    // So with combination of both, you can have one data that will havemultiple owners and
    // can change its data as well
    let a = Rc::new(RefCell::new(String::from("abc")));
    let b = Rc::clone(&a);

    *b.borrow_mut() = String::from("Hello");
    println!("{:?}", a)
}
