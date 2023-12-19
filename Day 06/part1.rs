use std::io::Read;

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let time: Vec<u32> = input
    .lines()
    .nth(0)
    .unwrap()
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect();
  println!("{:?}", time);
  let distance: Vec<u32> = input
    .lines()
    .nth(1)
    .unwrap()
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect();
  println!("{:?}", distance);

  //   let error: i64 = time.iter().zip(distance.iter()).fold(1, |acc, (&t, &d)| {
  //     let count = (0..t).filter(|&j| (t - j) * j > d).count();
  //     acc * count as i64
  // });

  let mut error: i64 = 1;
  for i in 0..time.len() {
    let mut count = 0;
    let t = time[i];
    let d = distance[i];
    for j in 0..t {
      let aux_d = (t - j) * j;
      if aux_d > d {
        count += 1;
      }
    }
    error *= count;
  }
  println!("{}", error);
}
