use std::io;

fn main() {
  println!("Enter a number: ");

  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("failed to read input");

  let number: i128 = input.trim().parse().expect("enter a valid number");

  // conditions

  // multi branch conditions
  if number > 5 {
    println!("condition was true");
  } else if number != 0 {
    println!("condition was false but number is not zero");
  } else {
    println!("condition was false and number is zero");
  }

  // inline condition
  let condition = true;
  let x = if condition { 5 } else { -5 };

  println!("x is {x}");

  // won't work due to type mismatch
  // let x = if condition { 5 } else { "six" };

  // loops

  // loop syntax
  let mut count = 0;
  loop {
    println!("loop {count}");
    count += 1;

    if count == 3 {
      break;
    }
  }

  // returning values from loop
  let result = loop {
    count += 1;

    if count == 30 {
      break count / 2;
    }
  };

  println!("result is {result}"); // 15

  // loop labels
  let mut count = 0;
  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remaining = {remaining}");
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }

    count += 1;
  }

  println!("end count = {count}");

  // conditional loops
  while count >= 0 {
    println!("count = {count}");
    count -= 1;
  }
  println!("LIFTOFF!!!");

  // loops with arrays
  let arr: [u8; 5] = [10, 20, 30, 40, 50];
  let mut index = 0;

  // while syntax
  while index < arr.len() {
    println!("arr[{index}] = {}", arr[index]);
    index += 1;
  }
  // the previous code is slower due to runtime
  // checks for index within bounds

  // for-in syntax
  // this is faster due to no runtime checks
  // for out of bounds index
  for element in arr {
    print!("arr: ");
    print!("{element} ");
    println!();
  }

  // for-in over a range
  for num in (1..4).rev() {
    println!("{num}");
  }
  println!("LIFTOFF!!!");
}
