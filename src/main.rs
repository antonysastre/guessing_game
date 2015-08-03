extern crate rand;

use std::io;
use rand::Rng;

fn main() {
  println!("Guess a number!");

  let secret_number = rand::thread_rng().gen_range(1, 11);

  println!("Input your guess:");

  let mut guess = String::new();

  io::stdin().read_line(&mut guess)
    .ok()
    .expect("Failed to read line.");

  println!("The secret number is: {}.", secret_number);
}
