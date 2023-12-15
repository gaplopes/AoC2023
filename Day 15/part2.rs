use std::collections::HashMap;
use std::io::Read;

fn string_to_hash_values(input: &str) -> i64 {
  let mut hash_value: i64 = 0;
  fn ascii_to_int(c: char) -> i64 {
    c as i64
  }
  for c in input.chars() {
    hash_value = (hash_value + ascii_to_int(c)) * 17 % 256;
  }
  return hash_value;
}

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let mut registers: HashMap<String, i64> = HashMap::new();
  let mut boxes: Vec<Vec<(String, i32)>> = Vec::new();
  for _ in 0..256 {
    boxes.push(Vec::new());
  }

  // Remove newlines
  input = input.replace("\n", "");
  // Split line strings by commas
  let mut line_strings = input.split(",");
  while let Some(line_string) = line_strings.next() {
    if line_string.contains("=") {
      let str_split: Vec<&str> = line_string.split("=").collect();
      let label = str_split[0];
      let hash_value = string_to_hash_values(label);
      let focal = str_split[1].parse::<i32>().unwrap();
      // println!("label: {}, focal: {}", label, focal);
      let box_number = registers.entry(label.to_string()).or_insert(-1);
      if *box_number == -1 {
        boxes[hash_value as usize].push((label.to_string(), focal));
        registers.insert(label.to_string(), hash_value);
      } else {
        let box_number_usize = *box_number as usize;
        for i in 0..boxes[box_number_usize].len() {
          if boxes[box_number_usize][i].0 == label.to_string() {
            boxes[box_number_usize][i].1 = focal;
          }
        }
      }
    } else if line_string.contains("-") {
      let label = line_string.split_at(line_string.find("-").unwrap()).0;
      // println!("label: {}", label);
      let box_number = registers.entry(label.to_string()).or_insert(-1);
      if *box_number != -1 {
        boxes[*box_number as usize].retain(|&(ref x, _)| x != label);
        registers.remove(label);
      }
    }
  }

  let mut sum: i64 = 0;
  for (i, b) in boxes.iter().enumerate() {
    // println!("{}: {:?}", i, b);
    for (j, value) in b.iter().enumerate() {
      sum += (1 + i as i64) * (1 + j as i64) * (value.1 as i64);
    }
  }
  println!("sum: {}", sum);
}
