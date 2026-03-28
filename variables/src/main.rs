const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
  // Mutability
  let mut x = 5;
  println!("The value of x is: {x}");
  x = 4;
  println!("The value of x is: {x}");
  x = 3;
  println!("The value of x is: {x}");

  // Scope
  {
    // Shadowing
    let x = x * 2;
    println!("The value of x in the inner scope is: {x}");
  }

  println!("The value of x outside the inner scope is: {x}");

  // Scoped declaration
  let y = {
    println!("The value of constant is: {THREE_HOURS_IN_SECONDS}");
    let x = 9;
    x - 1
  };

  println!("The value of y is: {y}");
  println!("The value of x is: {x}");

  // Shadowing with type change
  let spaces = "   ";
  let spaces = spaces.len();

  println!("The value of spaces is: {spaces}");
}
