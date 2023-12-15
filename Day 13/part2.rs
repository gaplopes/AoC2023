use std::cmp::min;
use std::io::Read;

fn count_differences(s1: &String, s2: &String) -> i32 {
  let mut diff = 0;
  for i in 0..s1.len() {
    if s1.chars().nth(i) != s2.chars().nth(i) {
      diff += 1;
    }
  }
  return diff;
}

fn process_mirror(mirror: &mut Vec<String>) -> i32 {
  // println!("{}", mirror.len());
  // for (i, line) in mirror.iter().enumerate() {
  //   println!("{}, {}", i, line);
  // }
  for i in 1..mirror.len() {
    let diff = count_differences(&mirror[i], &mirror[i - 1]);
    if diff == 0 {
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
      let diff_aux = count_differences(&prev_str, &next_str);
      if diff_aux == 1 {
        // println!("{} {}", i, length);
        return i as i32;
      }
    } else if diff == 1 {
      let save_str_curr = mirror[i].clone();
      let save_str_prev = mirror[i - 1].clone();

      fn try_mirror(mirror: &Vec<String>, i: usize) -> i32 {
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
        return -1;
      }

      mirror[i] = save_str_curr.clone();
      mirror[i - 1] = save_str_curr.clone();
      let result = try_mirror(&mirror, i);
      if result != -1 {
        return result;
      }
      mirror[i] = save_str_prev.clone();
      mirror[i - 1] = save_str_prev.clone();
      let result = try_mirror(&mirror, i - 1);
      if result != -1 {
        return result;
      }

      mirror[i] = save_str_curr.clone();
      mirror[i - 1] = save_str_prev.clone();
    } else {
      continue;
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
    let (mut rows, mut columns) = matrix_rows_columns(&mirror);
    let row = process_mirror(&mut rows);
    let column = process_mirror(&mut columns);
    // println!("{} {}", row, column);
    sum += (row as i64 * 100) + column as i64;
  }

  println!("{}", sum);
}
