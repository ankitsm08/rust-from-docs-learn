use std::io::{self, Write};

fn main() {
  print!("Enter a number: ");
  io::stdout().flush().expect("Failed to flush buffer");

  let mut input = String::new();

  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line.");

  let n: i32 = input.trim().parse().expect("Enter a valid number.");

  println!("number {n} Fibonacci number is {}", fibonacci(n));
}

fn fibonacci(n: i32) -> i32 {
  if n <= 1 {
    return n;
  }
  return fibonacci(n - 1) + fibonacci(n - 2);
}
