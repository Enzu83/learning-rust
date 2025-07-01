use std::env;
use std::io;

fn main() {
    match get_chapter_number() {
        1 => chapter_1(),
        2 => chapter_2(),
        unknown => panic!("Function for chapter {} doesn't exist", unknown),
    }
}

fn get_chapter_number() -> u8 {
    let args: Vec::<String> = env::args().collect();
    args[1].parse::<u8>().expect("Wrong argument for a chapter number")
}

fn chapter_1() {
    println!("Hello, world!");
}

fn chapter_2() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
