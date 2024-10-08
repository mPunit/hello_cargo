use core::str;

//String Slices
fn main() {
    let mut s = String::from("this is my first word");

    let word = return_first_word(&s);

    println!("{}", word);

    s.clear();
}

fn return_first_word(s: &String) -> &str {
    let b = s.as_bytes();

    for (i, &j) in b.iter().enumerate() {
        if j == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
