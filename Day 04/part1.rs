use std::io::Read;

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let mut sum_total = 0;
  for line in input.lines() {
    // Each line is a string similar to 'Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53'
    // Split string by : and |
    let mut parts = line.split(|c| c == ':' || c == '|');
    // Extract card ID
    let card = parts.next().unwrap();
    let card_id = card
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
    // Intersection of winning numbers and player numbers
    let mut intersection: Vec<u32> = Vec::new();
    for number in winning_numbers_list {
      if player_numbers_list.contains(&number) {
        intersection.push(number);
      }
    }
    let mut sum = 0;
    for _number in intersection {
      if sum == 0 {
        sum = 1;
      } else {
        sum *= 2;
      }
    }
    // println!("{}", sum);
    sum_total += sum;
  }
  println!("{}", sum_total);
}
