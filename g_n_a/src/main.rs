use std::cmp::Ordering;
use std::io;
use std::{thread, time};
use std::time::Duration;

use rand::prelude::*;

const MILLISSECONDS_750: Duration = time::Duration::from_millis(750);

fn main() {
    let mut floor: u128 = 0;
    println!("This is a test of a binary search algorithm searching the random number sorted in a specific range\nwhich maximum is...\n>>> ");
    let mut roof: String = String::new();
    io::stdin()
        .read_line(&mut roof)
        .expect("Failed to read number");
    let mut roof: u128 = match roof.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };

    let secret_number: u128 = rand::rng().random_range(1..=roof);
    dbg!(secret_number);
    let mut counter: u16 = 0;

    loop {
        if roof == 1 {
            println!("kaboom! error error error Rats!Rats!Rats!"); break;
        };
        
        

        let algorithms_guess: u128 = (roof + floor) / 2; counter += 1;
        
        let index: i8 = match algorithms_guess.cmp(&secret_number) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        };

        match index {
            -1 => { // below
                floor = algorithms_guess;
                println!("It guessed: {algorithms_guess};\nIt is below the target!");
                lil_pause();
                continue;
            },
            1 => { // above
                roof = algorithms_guess;
                println!("It guessed: {algorithms_guess};\nIt is above the target!");
                lil_pause();
                continue;
            }
            0 => { // the right guess
                println!("It has founded the number! It was {algorithms_guess} ({secret_number}) :D !!!!\nIt took {counter} guesses.");
                break;
            },
            _ => {println!("Panic!"); break;},
        };
    };
}
    
fn lil_pause() {
   thread::sleep(MILLISSECONDS_750);
}
