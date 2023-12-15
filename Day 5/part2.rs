use std::io::Read;

fn process_intervals(
  intervals: &Vec<(i64, i64)>,
  states_map: &Vec<(i64, i64, i64)>,
) -> Vec<(i64, i64)> {
  // Create a queue of intervals
  let mut queue: Vec<(i64, i64)> = Vec::new();
  for interval in intervals.iter() {
    queue.push(*interval);
  }
  // Process each interval
  let mut processed_intervals: Vec<(i64, i64)> = Vec::new();
  while let Some(interval) = queue.pop() {
    // println!("Checking interval {}..{}", interval.0, interval.1);
    let mut processed = false;
    for j in 0..states_map.len() {
      let (destination, source, length) = states_map[j];
      let offset: i64 = destination - source;
      let state_interval = (source, source + length - 1);
      // Verify if the interval overlaps with the state interval, and if so, split the interval
      if interval.0 >= state_interval.0 && interval.1 <= state_interval.1 {
        // Interval is contained in state interval
        // println!("Interval {}..{} is contained in state interval {}..{}", interval.0, interval.1, state_interval.0, state_interval.1);
        processed_intervals.push((interval.0 + offset, interval.1 + offset));
        processed = true;
        break;
      } else {
        if interval.0 >= state_interval.0 && interval.0 <= state_interval.1 {
          // Interval starts inside state interval, but ends outside
          // println!("Interval {}..{} starts inside state interval {}..{}", interval.0, interval.1, state_interval.0, state_interval.1);
          processed_intervals.push((interval.0 + offset, state_interval.1 + offset));
          queue.push((state_interval.1 + 1, interval.1));
          processed = true;
          break;
        } else if interval.1 >= state_interval.0 && interval.1 <= state_interval.1 {
          // Interval ends inside state interval, but starts outside
          // println!("Interval {}..{} ends inside state interval {}..{}", interval.0, interval.1, state_interval.0, state_interval.1);
          processed_intervals.push((state_interval.0 + offset, interval.1 + offset));
          queue.push((interval.0, state_interval.0 - 1));
          processed = true;
          break;
        } else if interval.0 <= state_interval.0 && interval.1 >= state_interval.1 {
          // Interval contains state interval
          // println!("Interval {}..{} contains state interval {}..{}", interval.0, interval.1, state_interval.0, state_interval.1);
          let new_interval_one = (interval.0, state_interval.0 - 1);
          let new_interval_two = (state_interval.1 + 1, interval.1);
          queue.push(new_interval_one);
          queue.push(new_interval_two);
          processed_intervals.push((state_interval.0 + offset, state_interval.1 + offset));
          processed = true;
          break;
        }
      }
    }
    if !processed {
      processed_intervals.push(interval);
    }
  }
  return processed_intervals;
}

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  // - seeds
  let mut lines = input.lines();
  let seeds_line = lines.next().unwrap();
  let pair_seeds: Vec<i64> = seeds_line
    .split(|c| c == ':')
    .nth(1)
    .unwrap()
    .split(|c| c == ' ')
    .filter(|s| !s.is_empty())
    .map(|s| s.parse::<i64>().unwrap())
    .collect();
  let mut intervals: Vec<(i64, i64)> = Vec::new();
  for i in 0..pair_seeds.len() / 2 {
    let seed1 = pair_seeds[i * 2];
    let seed2 = pair_seeds[i * 2 + 1];
    intervals.push((seed1, seed1 + seed2 - 1));
  }
  // println!("Intervals: {:?}", intervals);

  // Skip second line (empty)
  lines.next();
  // - process maps
  lines.next(); // Skip header
  let mut states_map: Vec<(i64, i64, i64)> = Vec::new();
  while let Some(line) = lines.next() {
    if line.is_empty() {
      lines.next(); // Skip empty line
      intervals = process_intervals(&intervals, &states_map);
      states_map.clear();
      // println!();
      // println!("Intervals: {:?}", intervals);
      // println!();
      continue;
    }
    let map: Vec<i64> = line
      .split(|c| c == ' ')
      .filter(|s| !s.is_empty())
      .map(|s| s.parse::<i64>().unwrap())
      .collect();
    // println!("Map state: {:?}", map);
    states_map.push((map[0], map[1], map[2]));
  }

  // - process last state
  intervals = process_intervals(&intervals, &states_map);

  let mut min_location = std::i64::MAX;
  for interval in intervals {
    if interval.0 < min_location {
      min_location = interval.0;
    }
  }
  println!("Min location: {}", min_location);
}
