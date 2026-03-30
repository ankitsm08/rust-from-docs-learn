/* opt in for debug traits
 * one of which is printing structs
 */
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "the area of the rectangle is {} square pixels",
    area(&rect1)
  );

  /* because of the #[derive(Debug)] attribute
   * we can print the struct, else we would get error
   * `Rectangle` doesn't implement `std::fmt::Display`
   */
  // :? or :#? implies to output in debug format
  println!("the rect was\n{rect1:?}");
  // :#? implies to print in more readable format
  println!("the rect was\n{rect1:?}");

  // another way to debug is using dbg!
  let scale = 2;
  let rect2 = Rectangle {
    width: dbg!(30 * dbg!(scale)),
    height: 50,
  };

  /* dbg! takes ownership of expression
   * so, pass the reference.
   * dbg! prints to stderr
   * whereas println! prints to stdout
   */
  dbg!(&rect2);
}

fn area(rect: &Rectangle) -> u32 {
  return rect.width * rect.height;
}
