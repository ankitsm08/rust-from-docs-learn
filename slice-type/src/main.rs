fn main() {
  let mut s = String::from("this is a string");

  // non-slice
  let w = first_word(&s);
  s.clear();
  // now w is invalid logically

  println!("the first word ends at index {w}");

  /* fn second_word(s: &String) -> (usize, usize)
   * will have the same issue and even more complex to sync
   * rust solves this by using `string slices`
   */

  let mut s = String::from("this is a string");

  // this is a pointer that points to start of `s`
  // but has len = 4
  let this = &s[..4];

  let is = &s[5..7];
  let a = &s[8..9];
  let string = &s[10..];

  println!("{this} {is} {a} {string}");

  // string slices are of type `&str`
  let first_word = first_word_slice(&s);

  // now, this will result in error until `first_word` in in scope
  // s.clear()

  println!("{first_word}");

  // but now its fine since borrow is out of scope
  s.clear();

  // general slices (arrays)
  let a = [1, 2, 3, 4, 5];

  let slice = &a[1..3];

  assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }

  return s.len();
}

fn first_word_slice(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &byte) in bytes.iter().enumerate() {
    if byte == b' ' {
      return &s[..i];
    }
  }

  return &s[..];
}
