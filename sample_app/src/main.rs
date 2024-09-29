fn main() {
    let a = Box::new(String::from("ABCD"));

    hello(&a);
}

fn hello(name: &str) {
    println!("{name}");
}
