use std::io::Read;

fn process_seed(seed: u64, states_map: &[(u64, u64, u64)]) -> u64 {
  states_map
    .iter()
    .find(|&&(_destination, source, length)| seed >= source && seed <= source + length)
    .map_or(seed, |&(destination, source, _)| {
      destination + seed - source
    })
}

fn main() {
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let mut lines = input.lines();
  let seeds_line = lines.next().unwrap();
  let mut seeds: Vec<u64> = seeds_line
    .split(':')
    .nth(1)
    .unwrap()
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

  lines.next(); // Skip second line (empty)
  lines.next(); // Skip header

  let mut states_map: Vec<(u64, u64, u64)> = Vec::new();
  while let Some(line) = lines.next() {
    if line.is_empty() {
      lines.next(); // Skip empty line
      seeds
        .iter_mut()
        .for_each(|seed| *seed = process_seed(*seed, &states_map));
      states_map.clear();
      continue;
    }
    let map: Vec<u64> = line
      .split_whitespace()
      .map(|s| s.parse().unwrap())
      .collect();
    states_map.push((map[0], map[1], map[2]));
  }

  seeds
    .iter_mut()
    .for_each(|seed| *seed = process_seed(*seed, &states_map));

  let min_location = seeds.iter().min().unwrap();
  println!("Min location: {}", min_location);
}
