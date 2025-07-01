use std::{
    env, 
    io,
    cmp::Ordering,
};

use rand::Rng;

fn main() {
    match get_chapter_number() {
        1 => chapter_1(),
        2 => chapter_2(),
        3 => chapter_3(),
        unknown => panic!("Function for chapter {} doesn't exist", unknown),
    }
}

fn get_chapter_number() -> u8 {
    let args: Vec::<String> = env::args().collect();

    if args.len() != 2 {
        panic!("No argument has been provided for selecting a chapter")
    }

    args[1]
        .parse::<u8>()
        .expect("Wrong argument for a chapter number")

}

fn chapter_1() {
    println!("Hello, world!");
}

fn chapter_2() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

fn chapter_3() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let tup = (500, 6.4, 1);

    let (_, y, _) = tup;

    println!("The value of y is: {y}");

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}