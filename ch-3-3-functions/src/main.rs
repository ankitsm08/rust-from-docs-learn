fn main() {
  println!("called `main()`");

  // void functions
  this_is_a_function();

  // functions with parameters
  this_is_x(1);
  print_labeled_measurement(456, 'm');

  // Returning functions
  println!("`five()` -> {}", five());
  let plus_one_of_five = plus_one(five());
  println!("`plus_one()` -> {}", plus_one_of_five);
}

fn this_is_a_function() {
  println!("called `this_is_a_function()`");
}

fn this_is_x(x: i32) {
  println!("called `this_is_x({x})`");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("called `print_labeled_measurement({value}, {unit_label})`");
}

fn five() -> i32 {
  return 5;
}

fn plus_one(x: i32) -> i32 {
  // This is an expression so its a valid return statement
  x + 1
}
