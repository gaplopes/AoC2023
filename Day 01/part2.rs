use std::io::Read;

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  // Vector of strings "one", "two", "three", ..., "nine"
  let numbers_written = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
  let numbers = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];

  let mut sum = 0;
  // For each line find the first number starting from left
  for line in input.lines() {
    // First number from right
    let mut first = '0';
    for (_i, c) in line.chars().enumerate() {
      if c.is_digit(10) {
        first = c;
        break;
      }
      let mut found = false;
      for (j, n) in numbers_written.iter().enumerate() {
        if _i + n.len() <= line.len() && &line[_i.._i + n.len()] == *n {
          first = numbers[j];
          found = true;
          break;
        }
      }
      if found {
        break;
      }
    }
    // println!("First found: {}", first);
    // First number from left
    let mut last = '0';
    for (_i, c) in line.chars().rev().enumerate() {
      if c.is_digit(10) {
        last = c;
        break;
      }
      let mut found = false;
      for (j, n) in numbers_written.iter().enumerate() {
        if _i + n.len() <= line.len() && &line[line.len() - _i - n.len()..line.len() - _i] == *n {
          last = numbers[j];
          found = true;
          break;
        }
      }
      if found {
        break;
      }
    }
    // println!("Last found: {}", last);
    // Create a string from the first and last number
    let mut s = String::new();
    s.push(first);
    s.push(last);
    // println!("{}", s);
    // Convert the string to a number
    let n: u32 = s.parse().unwrap();
    // Add the number to the sum
    sum += n;
  }
  println!("{}", sum);
}
