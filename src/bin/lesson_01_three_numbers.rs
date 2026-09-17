use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let numbers: Vec<i64> = input
        .split_whitespace()
        .map(|value| value.parse().expect("Invalid integer"))
        .collect();

    println!("{}", numbers[0] + numbers[1] + numbers[2]);
    println!("{}", numbers[0].max(numbers[1]).max(numbers[2]));
    println!("{}", numbers[0].min(numbers[1]).min(numbers[2]));
}
