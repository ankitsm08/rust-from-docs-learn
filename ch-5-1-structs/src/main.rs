// normal structs
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs
struct Unit;

fn main() {
  // creating instances from structs
  let mut user = User {
    active: true,
    username: String::from("someuser"),
    email: String::from("someuser@example.com"),
    sign_in_count: 1,
  };

  user.email = String::from("another@email.com");
  println!("{}", user.email);

  let user1 = build_user(
    String::from("builduser"),
    String::from("builduser@example.com"),
  );
  println!("{}", user1.username);

  let user2 = build_user_shorthand(
    String::from("buildusersh"),
    String::from("buildusersh@example.com"),
  );
  println!("{}", user2.active);

  let user3 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("newuser@example.com"),
    sign_in_count: user1.sign_in_count,
  };
  /* now user1 is not valid becuase user1.username
   * ownership has been transferred to user3.username
   * this will give `borrow of moved value` error
   * println!("{}", user1.username);
   */
  println!("{}", user3.sign_in_count);

  // struct update syntax
  // .. = fiels not explicitly set
  let user4 = User {
    email: String::from("newusersu@example.com"),
    ..user2
  };
  // now user2.username is also no longer valid
  println!("{}", user4.username);

  // creating instances of tuple structs
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);

  // tuple destructuring works for tuple structs too
  let Color(r, g, b) = black;
  let Point(x, y, z) = origin;

  println!("r: {r}; g: {g}; b: {b}");
  println!("x: {x}; y: {y}; z: {z}");

  // unit-like structs
  let _unit = Unit;
  // no data, only behavior implementation
}

fn build_user(username: String, email: String) -> User {
  return User {
    active: true,
    username: username,
    email: email,
    sign_in_count: 1,
  };
}

fn build_user_shorthand(username: String, email: String) -> User {
  /* uses build init shorthand
   * where if parameter names and struct
   * field names are same, no need to repeat
   */
  return User {
    active: true,
    username,
    email,
    sign_in_count: 1,
  };
}
