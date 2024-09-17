fn main() {
    let input = String::from("1211");
    println!(
        "It is {:?} that the given string is palindrome",
        palindrome(input)
    );
}

fn palindrome(input: String) -> bool {
    let mut is_palindrome = true;
    if input.len() == 0 {
        is_palindrome = true;
    } else {
        let mut last = input.len() - 1;
        let mut first = 0;

        let my_vec = input.as_bytes();

        while first < last {
            if my_vec[first] != my_vec[last] {
                is_palindrome = false;
                break;
            }

            first += 1;
            last -= 1;
        }
    }
    is_palindrome
}
