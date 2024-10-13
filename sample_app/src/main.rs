fn main() {
    let mut strings = vec![
        String::from("first"),
        String::from("apple"),
        String::from("ball"),
    ];

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    for word in &mut strings {
        let first_letter = word.chars().next().unwrap();
        if (vowels.contains(&first_letter.to_ascii_lowercase())) {
            word.push_str("-hay");
        } else {
            let mut characters = word.chars();
            let _first_letter = characters.next();
            let remaining: String = characters.collect();
            word.clear();
            word.push_str(&remaining);
            word.push('-');
            word.push(first_letter);
            word.push_str("ay");
        }
    }

    println!("{strings:?}")
}
