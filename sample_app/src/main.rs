use core::num;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut result = 0;

    let result: i32 = numbers
        .iter()
        .filter(|&&num| num % 2 == 0)
        .map(|&num| num * num)
        .sum();

    println!("Result without combinators: {}", result);

    let a = &numbers;
    let b = &a;

    println!("{:?}", &a)
}
