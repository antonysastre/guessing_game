extern crate rand;

use rand::Rng;

fn main() {
  println!("Guess a number!");

  let secret_number = rand::thread_rng().gen_range(1, 11);

  println!("The secret number is: {}.", secret_number);
}
