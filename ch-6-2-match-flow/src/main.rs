#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
  // -- more --
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  // binding to value
  Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
  // matches must be exhaustive
  match coin {
    Coin::Penny => {
      println!("Lucky Penny!");
      1
    }
    Coin::Nickel => 5,
    Coin::Dime => 10,
    // handling values on match
    Coin::Quarter(state) => {
      println!("State quarter from {state:?}!");
      25
    }
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    // option type matching
    Some(i) => Some(i + 1),
  }
}

fn main() {
  let coin_p = Coin::Penny;
  let coin_n = Coin::Nickel;
  let coin_d = Coin::Dime;
  let coin_q1 = Coin::Quarter(UsState::Alabama);
  let coin_q2 = Coin::Quarter(UsState::Alaska);

  for coin in [coin_p, coin_n, coin_d, coin_q1, coin_q2].iter() {
    println!("{}", value_in_cents(coin));
  }

  let a = Some(5);
  let b: Option<i32> = None;

  println!("{:?}", plus_one(a));
  println!("{:?}", plus_one(b));

  // catch-all pattern
  let dice_roll = 9;
  match dice_roll {
    // return unit value to do nothing
    0 => (),
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    // we can use `_` instead of `other`
    other => move_player(other),
  }

  fn add_fancy_hat() {}
  fn remove_fancy_hat() {}
  fn move_player(_num_spaces: u8) {}
}
