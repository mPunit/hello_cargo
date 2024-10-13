use core::{hash, str};
use std::{collections::HashMap, mem};

use std::collections::hash_map;

#[derive(Debug)]
struct User<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 5,
    };

    dbg!(&user1);
    println!("{:?}", user1);

    let a = [Message::Move { x: 1, y: 2 }, Message::Quit];

    for b in "Зд".bytes() {
        println!("{b}");
    }

    let mut hashes = HashMap::new();

    hashes.insert(1, "foo");
    hashes.insert(2, "bar");

    for a in hashes.keys() {
        println!("{a}");
    }

    println!("{:?}", hashes);

    let v = vec!["sdf", "sdfsf", "SDfsdF"];

    match v.get(2) {
        Some(n) => println!("{n}"),
        None => println!("Failure"),
    }
}
