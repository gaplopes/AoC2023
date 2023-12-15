use std::io::Read;

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let matrix: Vec<&str> = input.lines().collect();
  let mut sum = 0;
  for (i, l) in matrix.iter().enumerate() {
    let line = *l;
    // println!("{}: {}", i, line);
    let mut j : i32 = 0;
    let size = line.len();
    while j < size as i32 {
      // Access element j of line i
      if line.chars().nth(j as usize).unwrap().is_digit(10) {
        let mut number = String::new();
        let current_row : i32 = i as i32;
        let start_col : i32 = j;
        while j < size as i32 && line.chars().nth(j as usize).unwrap().is_digit(10) {
          number.push(line.chars().nth(j as usize).unwrap());
          j += 1;
        }
        let end_col : i32 = j - 1;
        let n: u32 = number.parse().unwrap();
        // println!("{}: {} - {} = {}", i, start_col, end_col, n);
        // For each surrounding cell of the number
        let mut symbol = false;
        for row in (current_row - 1)..(current_row + 2) {
          for col in (start_col - 1)..(end_col + 2) {
            // println!("{}: {}", row, col);
            if row >= 0 && row < matrix.len() as i32 && col >= 0 && col < line.len() as i32 {
              if !matrix[row as usize].chars().nth(col as usize).unwrap().is_digit(10) && matrix[row as usize].chars().nth(col as usize).unwrap() != '.' {
                symbol = true;
              }
            }
          }
        }
        if symbol {
          sum += n;
        }
      } else {
        j += 1;
      }
    }
  }
  println!("Sum: {}", sum);
}
