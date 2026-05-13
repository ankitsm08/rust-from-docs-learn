#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
  // -- more --
}

impl UsState {
  fn existed_in(&self, year: u16) -> bool {
    match self {
      UsState::Alabama => year > 1819,
      UsState::Alaska => year > 1959,
      // -- more --
    }
  }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
  // concise let ... else form
  // binds `state` to outer scope
  let Coin::Quarter(state) = coin else {
    return None;
  };

  if state.existed_in(1900) {
    Some(format!("{state:?} is pretty old, for America"))
  } else {
    Some(format!("{state:?} is relatively new"))
  }
}

enum Coin {
  Penny,
  // Nickel,
  // Dime,
  Quarter(UsState),
}

fn main() {
  let config_max = Some(3u8);
  match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
  }

  // this can be written in a more
  // concise form using if let
  let config_max = Some(3u8);
  if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
  }

  // another example
  let mut count = 0;
  let coin_p = Coin::Penny;
  let coin = Coin::Quarter(UsState::Alaska);
  match coin {
    Coin::Quarter(ref state) => println!("State quarter from {state:?}!"),
    _ => count += 1,
  }
  println!("count = {count}");

  // can be concisely written
  // using if let ... else
  let coin = Coin::Quarter(UsState::Alabama);
  if let Coin::Quarter(ref state) = coin {
    println!("State quarter from {state:?}!");
  } else {
    count += 1;
  }
  println!("count = {count}");

  println!("{:?}", describe_state_quarter(coin));
  println!("{:?}", describe_state_quarter(coin_p));
}
