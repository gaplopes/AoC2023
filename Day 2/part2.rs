use std::io::Read;
use std::collections::HashMap;

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let mut sum_products = 0;

  for line in input.lines() {
    // Each line is a string similar to 'Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green; ...'
    // Split string by : and ;
    let mut parts = line.split(|c| c == ':' || c == ';');
    // Extract game ID
    let game = parts.next().unwrap();
    let game_id = game.split(|c| c == ' ').nth(1).unwrap().parse::<u32>().unwrap();
    // println!("Game {}", game_id);
    // Get subsets from parts, skip the first element (already skipped by parts.next())
    let mut map_max = HashMap::new();
    map_max.insert("red", 0);
    map_max.insert("green", 0);
    map_max.insert("blue", 0);
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
      map_max.insert("red", std::cmp::max(map_max["red"], map["red"]));
      map_max.insert("green", std::cmp::max(map_max["green"], map["green"]));
      map_max.insert("blue", std::cmp::max(map_max["blue"], map["blue"]));
    }
    sum_products += map_max["red"] * map_max["green"] * map_max["blue"];
    // Print the number of reds, greens and blues
    // println!("Reds: {}, Greens: {}, Blues: {}", map["red"], map["green"], map["blue"]);
  }
  println!("{}", sum_products);
}
