use core::str;
use std::mem;

fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..];

    assert_eq!(std::mem::size_of_val(&arr), 12);
    assert_eq!(std::mem::size_of_val(slice), 12);

    assert_eq!(std::mem::size_of_val(&slice), 16);
    println!("Success!");
}
