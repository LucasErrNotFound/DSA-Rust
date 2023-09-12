use text_io::read;

fn main() {
    print!("Enter a number: ");
    let number: i32 = read!("{}\n");

    println!("Multiplication table of {number}: {}", product(number, 1));
}

fn product(number: i32, multiplier: i32) -> i32 {
    let answer = number * multiplier;

    if multiplier < 10 {
        product(number, multiplier + 1)
    } else {
        answer
    }
}
