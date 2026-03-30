use std::io::{self, Write};

fn main() {
  print!("Enter a temperature in celcius: ");
  io::stdout().flush().expect("Failed to flush buffer");

  let mut input = String::new();

  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line.");

  let c: f64 = input.trim().parse().expect("Enter a valid temperature.");

  println!("{c} deg C in Farenheit is {}", celcius_to_farenheit(c));
}

fn celcius_to_farenheit(c: f64) -> f64 {
  return (c * 9.0 / 5.0) + 32.0;
}
