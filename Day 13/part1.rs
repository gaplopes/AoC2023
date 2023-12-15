use std::cmp::min;
use std::io::Read;

fn process_mirror(mirror: &Vec<String>) -> i32 {
  // println!("{}", mirror.len());
  // for (i, line) in mirror.iter().enumerate() {
  //   println!("{}, {}", i, line);
  // }
  for i in 1..mirror.len() {
    if mirror[i] == mirror[i - 1] {
      let length = min(i, mirror.len() - i);
      let mut prev_str: String = "".to_string();
      for j in i - length..i {
        prev_str += &mirror[j];
      }
      // println!("{}", prev_str);
      let mut next_str: String = "".to_string();
      for j in i..i + length {
        next_str = mirror[j].clone() + &next_str;
      }
      // println!("{}", next_str);
      if prev_str == next_str {
        // println!("{} {}", i, length);
        return i as i32;
      }
    }
  }
  return 0;
}

fn matrix_rows_columns(mirror: &Vec<String>) -> (Vec<String>, Vec<String>) {
  let rows: Vec<String> = mirror.clone();
  let columns: Vec<String> = (0..mirror[0].len())
    .map(|i| {
      let mut column = "".to_string();
      for row in &rows {
        column += &row[i..i + 1];
      }
      column
    })
    .collect();
  (rows, columns)
}

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let mut sum: i64 = 0;
  let lines = input.lines();
  let mut mirrors: Vec<Vec<String>> = Vec::new();
  let mut mirror: Vec<String> = Vec::new();
  for line in lines {
    if line == "" {
      mirrors.push(mirror.clone());
      mirror.clear();
    } else {
      mirror.push(line.to_string());
    }
  }
  mirrors.push(mirror.clone());

  for mirror in mirrors {
    let (rows, columns) = matrix_rows_columns(&mirror);
    let row = process_mirror(&rows);
    let column = process_mirror(&columns);
    // println!("{} {}", row, column);
    sum += (row as i64 * 100) + column as i64;
  }

  println!("{}", sum);
}
