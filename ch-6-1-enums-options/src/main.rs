#![allow(dead_code)]

#[derive(Debug)]
enum IpAddrKind {
  V4,
  V6,
}

#[derive(Debug)]
struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

// or we can bundle data directly inside enums
#[derive(Debug)]
enum IpAddressBundle {
  V4(u8, u8, u8, u8),
  V6(String),
}

// or buncle separate structs inside enums
#[derive(Debug)]
struct Ipv4Addr {
  address: (u8, u8, u8, u8),
}
#[derive(Debug)]
struct Ipv6Addr {
  address: String,
}
#[derive(Debug)]
enum IpAddress {
  V4(Ipv4Addr),
  V6(Ipv6Addr),
}

/* defining each enum value as a struct won't
 * group them together under Message type
 */
#[derive(Debug)]
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

// implementation of enum
impl Message {
  fn call(&self) {
    println!("{:?}", self);
  }
}

fn main() {
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;

  println!("{:?}; {:?}", four, six);

  let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
  };
  let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
  };

  println!("{:#?}\n{:#?}", home, loopback);

  let home = IpAddressBundle::V4(127, 0, 0, 1);
  let loopback = IpAddressBundle::V6(String::from("::1"));

  println!("{:#?}\n{:#?}", home, loopback);

  let home = IpAddress::V4(Ipv4Addr {
    address: (127, 0, 0, 1),
  });
  let loopback = IpAddress::V6(Ipv6Addr {
    address: String::from("::1"),
  });

  println!("{:#?}\n{:#?}", home, loopback);

  // method calls on enum values
  let m = Message::Write(String::from("hello"));
  m.call();

  // option type (already included in prelude)
  let some_number = Some(5);
  let some_string = Some(String::from("a string"));
  let absent_number: Option<i32> = None;
  println!("{:?}\n{:?}\n{:?}", some_number, some_string, absent_number);

  // unwrapping
  let x = 5;
  let y = Some(5);
  let sum = x + y.unwrap_or(0);
  println!("{} + {:?} = {}", x, y, sum);
}
