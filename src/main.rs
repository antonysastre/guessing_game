extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess a number!");

  let secret_number = rand::thread_rng().gen_range(1, 11);

  println!("I'm thinking of a number.");

    println!("Input your guess:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
      .ok()
      .expect("Failed to read line.");

    let guess: u32 = guess.trim().parse()
      .ok()
      .expect("Please type a number!");

    println!("The secret number was: {}.", secret_number);

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => println!("You win!")
    }
}
