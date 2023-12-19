use std::collections::HashMap;
use std::io::Read;

fn hand_type(input: &str) -> i32 {
  let mut map = HashMap::new();
  for c in input.chars() {
    let counter = map.entry(c).or_insert(0);
    *counter += 1;
  }
  match map.len() {
    1 => 1, // "Five of a kind"
    2 => {
      if map.values().any(|&v| v == 4) {
        2 //"Four of a kind"
      } else {
        3 //"Full house"
      }
    }
    3 => {
      if map.values().any(|&v| v == 3) {
        4 // "Three of a kind"
      } else {
        5 // "Two pair"
      }
    }
    4 => 6, //"One pair",
    5 => 7, //"High card",
    _ => panic!("Invalid hand"),
  }
}

fn card_value(c: char) -> i32 {
  match c {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'J' => 11,
    'T' => 10,
    _ => c.to_digit(10).unwrap() as i32,
  }
}

fn sort_by_hand(hands: &mut Vec<(String, i32)>) {
  hands.sort_by(|a, b| {
    let hand_a = hand_type(&a.0);
    let hand_b = hand_type(&b.0);
    if hand_a == hand_b {
      for (c_a, c_b) in a.0.chars().zip(b.0.chars()) {
        if c_a != c_b {
          // return c_a.cmp(&c_b);
          return card_value(c_b).cmp(&card_value(c_a));
        }
      }
      panic!("Invalid hands");
    } else {
      hand_a.cmp(&hand_b)
    }
  });
  hands.reverse();
}

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let mut hands: Vec<(String, i32)> = Vec::new();
  for line in input.lines() {
    let mut parts = line.split_whitespace();
    let hand = parts.next().unwrap();
    let value = parts.next().unwrap().parse::<i32>().unwrap();
    hands.push((hand.to_string(), value));
  }

  // for (hand, value) in &hands {
  //   println!("{} {} {}", hand, value, hand_type(hand));
  // }

  sort_by_hand(&mut hands);

  let mut total_winnings: i64 = 0;
  for (i, hand) in hands.iter().enumerate() {
    let winnings = (i + 1) as i64 * hand.1 as i64;
    total_winnings += winnings;
    // println!("{} {} {}", hands[i].0, hands[i].1, winnings);
  }

  println!("Total winnings: {}", total_winnings);
}
