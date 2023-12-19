use std::io::Read;

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

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
    }
    // First number from left
    let mut last = '0';
    for (_i, c) in line.chars().rev().enumerate() {
      if c.is_digit(10) {
        last = c;
        break;
      }
    }
    // Create a string from the first and last number
    let mut s = String::new();
    s.push(first);
    s.push(last);
    // Convert the string to a number
    let n: u32 = s.parse().unwrap();
    // Add the number to the sum
    sum += n;
  }
  println!("{}", sum);
}
