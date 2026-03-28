use std::io;

#[allow(unused_variables)]
fn main() {
  // Data types
  // Explicitly state type when parsing
  let guess: u32 = "42".parse().expect("Not a number!");

  // integer
  let a = 36; // i32 - default
  let a: i8 = -36; // i8
  let a: i64 = 12345; // i64

  let a: u8 = 36; // u8
  let a: u64 = 12345; // u64

  // platform dependent
  let a: isize = -36;
  let a: usize = 36;

  // floating point
  let x = 2.0; // f64 - default
  let y: f32 = 3.0; // f32

  // addition
  let sum = 5 + 10;

  // subtraction
  let difference = 95.5 - 4.3;

  // multiplication
  let product = 4 * 30;

  // division
  let quotient = 56.7 / 32.2;
  let truncated = -5 / 3; // Results in -1

  // remainder
  let remainder = 43 % 5;

  // boolean
  let t = true;

  let f: bool = false; // with explicit type annotation

  // char (in rust chars are 4 bytes Unicode and not ASCI)
  let c = 'z';
  let z: char = 'ℤ'; // with explicit type annotation
  let heart_eyed_cat = '😻';

  // Compound types
  // tuples
  let tup: (i16, f32, char) = (100, 3.1415, 'P');
  let tup2 = (100, 3.1415, 'P'); // (i32, f64, char) - defaults
  let (x, y, z) = tup; // unpacking

  println!("The value of y is {y}");

  // access by index
  let x: (i32, f64, u8) = (500, 6.4, 1);

  let five_hundred = x.0;

  let six_point_four = x.1;

  let one = x.2;

  // arrays
  let a = [1, 2, 3, 4, 5];
  let months = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
  ];
  let b: [i16; 5] = [1, 2, 4, 8, 16];
  let c = [3; 5]; // [3, 3, 3, 3, 3]

  // access by index
  let first = a[0];
  let second = a[1];

  // example
  let a = [1, 2, 3, 4, 5];

  println!("Please enter an array index.");

  let mut index = String::new();

  io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

  let index: usize = index
    .trim()
    .parse()
    .expect("Index entered was not a number");

  // will panic at runtime if index >= a.len()
  let element = a[index];

  println!("The value of the element at index {index} is: {element}");
}
