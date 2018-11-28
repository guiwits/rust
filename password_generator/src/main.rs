extern crate rand;

use rand::Rng;
use std::env;

fn main() {
  let right: [char; 21] = [
    '6', '7', '8', '9', '0', '^', '&', '*', 'y', 'u', 'i', 'o', 'p', 'h', 'j', 'k', 'l', 'n', 'm',
    '{', '}',
  ];
  let left: [char; 25] = [
    '1', '2', '3', '4', '5', '!', '@', '#', '$', '%', 'q', 'w', 'e', 'r', 't', 'a', 's', 'd', 'f',
    'g', 'z', 'x', 'c', 'v', 'b',
  ];

  let args: Vec<String> = env::args().collect();

  if args.len() > 2 {
    panic!("This program only takes one argument, password length");
  }
  let len = &args[1];

  println!("Password length is {}", len);

  let length: u32 = match len.parse::<u32>() {
    Ok(n) => n,
    Err(_) => {
      panic!("error: argument is not an integer.");
    }
  };

  for p in 0..length {
    let mut rng = rand::thread_rng();
    let mut upper_case = rng.gen_range(0, length * 1000);

    // modulo 0 is the right side, 1 is left.
    // alternate between left and right.
    // don't upper case left since i hit shift with my left
    if p % 2 == 0 && upper_case % 2 == 0 {
      print!("{}", right[rng.gen_range(0, 21)].to_uppercase());
    } else if p % 2 == 0 && upper_case % 2 != 0 {
      print!("{}", right[rng.gen_range(0, 21)]);
    } else if p % 2 != 0 {
      print!("{}", left[rng.gen_range(0, 25)]);
    }
  }
}
