use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let query: &str = "Start:";
  let mut n: usize = 0;
  let mut start: bool = false;

  // dumb hack reading the file into two different variables
  // but one is a string and the other is a BufReader
  let mut f = File::open(filename).expect("file not found");
  let f1 = File::open(filename).expect("file not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("Something went wrong reading the file");

  let num_times: usize = contents.matches(query).count();

  let reader = BufReader::new(f1);
  for line in reader.lines() {
    let l = line.unwrap();
    if l.contains("Start:") {
      n = n + 1;
      if n == num_times {
        start = true;
      }
    } else if start == true {
      if l.contains("Send has taken more than 60 seconds") {
        println!("ERROR: Send has taken more than 60 seconds.");
        process::exit(2);
      }
    } else if num_times == 0 {
      if l.contains("Send has taken more than 60 seconds") {
        println!("ERROR: Send has taken more than 60 seconds.");
        process::exit(2);
      }
    }
  }
  println!("No issues found with the ActiveMQ WP broker.");
  process::exit(0);
}
