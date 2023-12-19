use std::collections::HashMap;
use std::io::Read;

fn hand_type(input: &str) -> i32 {
  let mut map = HashMap::new();
  let jokers = input.chars().filter(|&c| c == 'J').count();
  input
    .chars()
    .filter(|&c| c != 'J')
    .for_each(|c| *map.entry(c).or_insert(0) += 1);

  let max_count = *map.values().max().unwrap_or(&0);
  let unique_cards = map.len();

  match max_count + jokers {
    5 => 1, // "Five of a kind"
    4 => 2, // "Four of a kind"
    3 => {
      if unique_cards == 2 {
        3 // "Full house"
      } else {
        4 // "Three of a kind"
      }
    }
    2 => {
      if unique_cards == 3 {
        5 // "Two pair"
      } else {
        6 // "One pair"
      }
    }
    1 => 7, // "High card",
    _ => panic!("Invalid hand"),
  }
}

fn card_value(c: char) -> i32 {
  match c {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'J' => 1,
    'T' => 10,
    _ => c.to_digit(10).expect("Invalid card value") as i32,
  }
}

fn sort_by_hand(hands: &mut Vec<(String, i32)>) {
  hands.sort_by(|a, b| {
    let hand_a = hand_type(&a.0);
    let hand_b = hand_type(&b.0);
    if hand_a == hand_b {
      // Same hand type, sort by card value
      b.0.chars().map(card_value).cmp(a.0.chars().map(card_value))
    } else {
      hand_a.cmp(&hand_b)
    }
  });
  hands.reverse();
}

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin()
    .read_to_string(&mut input)
    .expect("Failed to read from stdin");

  let mut hands: Vec<(String, i32)> = input
    .lines()
    .map(|line| {
      let mut parts = line.split_whitespace();
      let hand = parts.next().expect("Missing hand").to_string();
      let value = parts
        .next()
        .expect("Missing value")
        .parse::<i32>()
        .expect("Invalid value");
      (hand, value)
    })
    .collect();

  sort_by_hand(&mut hands);

  let mut total_winnings: i64 = 0;
  for (i, hand) in hands.iter().enumerate() {
    let winnings = (i + 1) as i64 * hand.1 as i64;
    total_winnings += winnings;
    // println!("{} {} {}", hands[i].0, hands[i].1, winnings);
  }

  println!("Total winnings: {}", total_winnings);
}
