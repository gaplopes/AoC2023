use std::io::Read;

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let mut total_sum: i64 = 0;
  for line in input.lines() {
    let numbers: Vec<i32> = line
      .split_whitespace()
      .map(|x| x.parse().expect("Not a number!"))
      .collect();
    let mut differences: Vec<Vec<i32>> = Vec::new();
    differences.push(numbers.clone());
    loop {
      let mut difference: Vec<i32> = Vec::new();
      let mut aux_sum: i64 = 0;
      let last = differences.last().expect("Empty vector!");
      for i in 1..last.len() {
        difference.push(last[i] - last[i - 1]);
        aux_sum += numbers[i - 1] as i64;
      }
      if aux_sum == 0 || difference.is_empty() {
        break;
      }
      differences.push(difference);
    }
    let mut sum: i64 = 0;
    for difference in differences.iter().rev() {
      sum = *difference.first().expect("Empty vector!") as i64 - sum;
    }
    // println!("Sum: {}", sum);
    total_sum += sum;
  }
  println!("Total sum: {}", total_sum);
}
