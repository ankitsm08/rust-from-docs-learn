fn main() {
  /* we dont always want to transwer ownership everytime we pass
   * some value to a function, rust as a feature called `references`
   */
  let mut s = String::from("string");
  // here we pass a new pointer to s
  // thus never giving ownership of s to the function
  let len = calculate_length(&s);
  println!("length of `s` is {len}");

  // references are immutable by defualt
  let len = append_hello_and_calculate_length(&mut s);
  println!("length of appended `s` is {len}");

  /* in rust, we cannot create any other references to something
   * if a mutable reference has been created
   * this is to prevent data races
   */
  // let mut s = String::from("hello");
  //
  // let r1 = &s; // no problem
  // let r2 = &s; // no problem
  // let r3 = &mut s; // BIG PROBLEM
  //
  // println!("{r1}, {r2}, and {r3}");

  /* but, if we create mutable references after
   * all other references are gone, we are fine
   */
  let mut s = String::from("hello");

  let r1 = &s; // no problem
  let r2 = &s; // no problem
  println!("{r1} and {r2}");
  // Variables r1 and r2 will not be used after this point.

  let r3 = &mut s; // no problem
  println!("{r3}");

  // rust also prevents dangling references at compile time
}

fn calculate_length(s: &String) -> usize {
  return s.len();
}

fn append_hello_and_calculate_length(s: &mut String) -> usize {
  s.push_str("hello");
  return s.len();
}
