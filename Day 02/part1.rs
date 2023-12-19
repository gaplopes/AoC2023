use std::io::Read;
use std::collections::HashMap;

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let reds = 12;
  let greens = 13;
  let blues = 14;
  let mut sum_ids = 0;

  for line in input.lines() {
    // Each line is a string similar to 'Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green; ...'
    // Split string by : and ;
    let mut parts = line.split(|c| c == ':' || c == ';');
    // Extract game ID
    let game = parts.next().unwrap();
    let game_id = game.split(|c| c == ' ').nth(1).unwrap().parse::<u32>().unwrap();
    // println!("Game {}", game_id);
    let mut valid = true;
    // Get subsets from parts, skip the first element (already skipped by parts.next())
    for subset in parts {
      // Create a map with keys red, green and blue
      let mut map = HashMap::new();
      map.insert("red", 0);
      map.insert("green", 0);
      map.insert("blue", 0);
      // Split subset by , and space
      // println!("Subset: {}", subset);
      let subset_parts = subset.split(|c| c == ',');
      for (_i, part) in subset_parts.enumerate() {
        // println!("Part {}: {}", i, part);
        let mut part_parts = part.split(|c| c == ' ');
        let value : u32 = part_parts.nth(1).unwrap().parse().unwrap();
        let color = part_parts.nth(0).unwrap();
        map.insert(color, map[color] + value);
      }
      // Verify if the number of reds, greens and blues does not exceed the limit
      if map["red"] > reds || map["green"] > greens || map["blue"] > blues {
        // println!("Subset {} is invalid", subset);
        valid = false;
      }
    }
    if valid {
      sum_ids += game_id;
    }
    // Print the number of reds, greens and blues
    // println!("Reds: {}, Greens: {}, Blues: {}", map["red"], map["green"], map["blue"]);
  }
  println!("{}", sum_ids);
}
