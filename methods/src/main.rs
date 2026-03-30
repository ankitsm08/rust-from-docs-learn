#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  // associated functions

  // self arguments means this function is a "method"
  fn area(self: &Self) -> u32 {
    return self.width * self.height;
  }

  // &self is a shorthand for self: &Self
  fn diagonal(&self) -> f64 {
    let d2 = self.width * self.width + self.height * self.height;
    return (d2 as f64).sqrt();
  }

  fn width(&self) -> bool {
    return self.width > 0;
  }

  // no self argument means this method is accessible
  // using XYZ::function
  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
}

// implementation can be spread out into multiple impl blocks
impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    return self.width >= other.width && self.height >= other.height;
  }
}

fn main() {
  let rect = Rectangle {
    width: 30,
    height: 10,
  };

  println!("area of rect is {} square pixels", rect.area());
  println!("diagonal of rect is {} pixels long", rect.diagonal());

  if rect.width() {
    println!("rect has a nonzero width; it is {}", rect.width);
  }

  let rect_other = Rectangle {
    width: 20,
    height: 8,
  };

  println!("`rect_small` fit in `rect`? {}", rect.can_hold(&rect_other));
  println!("`rect` fit in `rect_small`? {}", rect_other.can_hold(&rect));

  let sq = Rectangle::square(24);
  println!("{sq:?}");
}
