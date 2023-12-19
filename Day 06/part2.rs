use std::io::Read;

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let time: i64 = input.lines().nth(0).unwrap().replace(" ", "").split(":").nth(1).unwrap().parse().unwrap();
  // println!("{}", time);
  let distance: i64 = input.lines().nth(1).unwrap().replace(" ", "").split(":").nth(1).unwrap().parse().unwrap();
  // println!("{}", distance);

  let count: i64 = (0..time).filter(|&j| (time - j) * j > distance).count() as i64;
  println!("{}", count);
}
