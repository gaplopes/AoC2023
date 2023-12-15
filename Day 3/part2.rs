use std::io::Read;

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();
  let matrix: Vec<&str> = input.lines().collect();

  // Map of tuple (i, j) with list of numbers
  let mut symbol_numbers : std::collections::HashMap<(i32, i32), Vec<u32>> = std::collections::HashMap::new();
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
              let cell = matrix[row as usize].chars().nth(col as usize).unwrap();
              if !cell.is_digit(10) && cell != '.' {
                symbol = true;
                if cell == '*' {
                  if symbol_numbers.contains_key(&(row, col)) {
                    symbol_numbers.get_mut(&(row, col)).unwrap().push(n);
                  } else {
                    symbol_numbers.insert((row, col), vec![n]);
                  }
                }
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
  let mut sum_gears = 0;
  for (_key, value) in symbol_numbers.iter() {
    // Verify if the vector contains only two numbers
    if value.len() == 2 {
      sum_gears += value[0] * value[1];
    }
  }
  println!("Sum of gears: {}", sum_gears);
}
