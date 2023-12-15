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
      for i in 1..differences.last().expect("Empty vector!").len() {
        difference.push(
          differences.last().expect("Empty vector!")[i]
            - differences.last().expect("Empty vector!")[i - 1],
        );
        aux_sum += numbers[i - 1] as i64;
      }
      if aux_sum == 0 || difference.len() == 0 {
        break;
      }
      differences.push(difference);
    }
    let mut sum: i64 = 0;
    for difference in differences {
      sum += *difference.last().expect("Empty vector!") as i64;
    }
    println!("Sum: {}", sum);
    total_sum += sum;
  }
  println!("Total sum: {}", total_sum);
}
