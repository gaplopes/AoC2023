use std::collections::HashMap;
use std::io::Read;

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let mut cards_map: Vec<(HashMap<u32, u32>, HashMap<u32, u32>)> = Vec::new();
  let mut cards_repeat: Vec<u32> = Vec::new();
  for line in input.lines() {
    // Each line is a string similar to 'Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53'
    // Split string by : and |
    let mut parts = line.split(|c| c == ':' || c == '|');
    // Extract card ID
    let card = parts.next().unwrap();
    let _card_id = card
      .split(|c| c == ' ')
      .filter(|s| !s.is_empty())
      .nth(1)
      .unwrap();
    // println!("Card {}", card_id);
    // Winning numbers
    let mut winning_numbers = parts.next().unwrap();
    // Split winning numbers by space
    winning_numbers = winning_numbers.trim();
    let mut winning_numbers_list: Vec<u32> = winning_numbers
      .split(|c| c == ' ')
      .filter(|s| !s.is_empty())
      .map(|s| s.parse::<u32>().unwrap())
      .collect();
    winning_numbers_list.sort();
    // Player numbers
    let mut player_numbers = parts.next().unwrap();
    // Split player numbers by space
    player_numbers = player_numbers.trim();
    let mut player_numbers_list: Vec<u32> = player_numbers
      .split(|c| c == ' ')
      .filter(|s| !s.is_empty())
      .map(|s| s.parse::<u32>().unwrap())
      .collect();
    player_numbers_list.sort();
    // Add to cards_map
    let mut card_map: (HashMap<u32, u32>, HashMap<u32, u32>) = (HashMap::new(), HashMap::new());
    for number in winning_numbers_list {
      if !card_map.0.contains_key(&number) {
        card_map.0.insert(number, 1);
      }
    }
    for number in player_numbers_list {
      if !card_map.1.contains_key(&number) {
        card_map.1.insert(number, 1);
      }
    }
    cards_map.push(card_map);
    cards_repeat.push(1);
  }

  for i in 0..cards_map.len() {
    let cards_i = cards_map[i].clone();
    let winning_numbers_map = cards_i.0;
    let player_numbers_map = cards_i.1;
    // Intersection of winning numbers and player numbers
    let mut intersection: Vec<u32> = Vec::new();
    for number in winning_numbers_map.keys() {
      if player_numbers_map.contains_key(number) {
        intersection.push(*number);
      }
    }
    let equal_count = intersection.len();
    for _j in 0..cards_repeat[i] {
      for k in i + 1..i + equal_count + 1 {
        if k >= cards_map.len() {
          break;
        }
        cards_repeat[k] += 1;
      }
    }
  }

  let mut repeats = 0;
  for card_repeat in cards_repeat {
    // println!("{}", card_repeat);
    repeats += card_repeat;
  }

  println!("{}", repeats);
}
