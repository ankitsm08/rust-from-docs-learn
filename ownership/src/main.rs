fn main() {
  {
    let s = String::from("hello");
    println!("{s}");
  }
  /* after s is out of scope, the memory is freed automatically
   * now, s is no longer valid
   * rust calls a special function called `drop`
   * in c++, this pattern of deallocating memory at end of lifetime
   * is called as RAII (Resource Acquisition is Initialization)
   */

  let x = 5;
  // value is copied over in new memory
  let y = x;

  println!("x = {x}; y = {y}");

  // String
  /* ptr ----------------> char[]
   * len
   * capacity
   */
  let str1 = String::from("string");
  // actual string is in heap, only the (pointer, len and capacity) is duplicated on stack
  let str2 = str1;
  // for str1, its ptr points to same location as str2's ptr does
  // so actual data is not duplicated on the heap
  println!("{str2}");
  /* but in rust, shallow copy invalidates the previous variable
   * now using str1 will result in an error
   * this operation is called `move` in rust
   */
  // This will give the error
  // `borrow of moved value`
  // println!("{str1}")

  let s = String::from("hello");
  takes_ownership(s);
  // now s is moved, so it cannot be used afterwards
  // takes_ownership(s);

  let i = 5;
  makes_copy(i);

  let s = gives_ownership();
  println!("{s}");

  let s2 = takes_and_gives_back_ownership(s);
  println!("{s2}");
}

fn takes_ownership(some_string: String) {
  println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
  println!("{some_integer}");
}

fn gives_ownership() -> String {
  let s = String::from("ownership");
  return s;
}

fn takes_and_gives_back_ownership(some_string: String) -> String {
  println!("{some_string}");
  return some_string;
}
