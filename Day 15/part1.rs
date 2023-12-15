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

  // Remove newlines
  input = input.replace("\n", "");
  // Split line strings by commas
  let mut line_strings = input.split(",");
  let mut sum: i64 = 0;
  while let Some(line_string) = line_strings.next() {
    let hash_value = string_to_hash_values(line_string);
    // println!("{}: {}", line_string, hash_value);
    sum += hash_value;
  }
  println!("sum: {}", sum);
}
