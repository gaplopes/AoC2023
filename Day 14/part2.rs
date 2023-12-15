use std::collections::HashMap;
use std::io::Read;

fn move_north(matrix: &mut Vec<Vec<char>>, width: usize, height: usize) {
  for h in 0..height {
    for w in 0..width {
      if matrix[h][w] != 'O' {
        continue;
      }
      matrix[h][w] = '.';
      // Move the O up until it hits a wall # or another O
      let mut h2 = h;
      while h2 > 0 && matrix[h2 - 1][w] == '.' {
        h2 -= 1;
      }
      matrix[h2][w] = 'O';
    }
  }
}

fn move_south(matrix: &mut Vec<Vec<char>>, width: usize, height: usize) {
  for h in (0..height).rev() {
    for w in 0..width {
      if matrix[h][w] != 'O' {
        continue;
      }
      matrix[h][w] = '.';
      // Move the O up until it hits a wall # or another O
      let mut h2 = h;
      while h2 < height - 1 && matrix[h2 + 1][w] == '.' {
        h2 += 1;
      }
      matrix[h2][w] = 'O';
    }
  }
}

fn move_west(matrix: &mut Vec<Vec<char>>, width: usize, height: usize) {
  for h in 0..height {
    for w in 0..width {
      if matrix[h][w] != 'O' {
        continue;
      }
      matrix[h][w] = '.';
      // Move the O up until it hits a wall # or another O
      let mut w2 = w;
      while w2 > 0 && matrix[h][w2 - 1] == '.' {
        w2 -= 1;
      }
      matrix[h][w2] = 'O';
    }
  }
}

fn move_east(matrix: &mut Vec<Vec<char>>, width: usize, height: usize) {
  for h in 0..height {
    for w in (0..width).rev() {
      if matrix[h][w] != 'O' {
        continue;
      }
      matrix[h][w] = '.';
      // Move the O up until it hits a wall # or another O
      let mut w2 = w;
      while w2 < width - 1 && matrix[h][w2 + 1] == '.' {
        w2 += 1;
      }
      matrix[h][w2] = 'O';
    }
  }
}

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  // println!("Original matrix:");
  // for row in matrix.iter() {
  //   println!("{}", row.iter().collect::<String>());
  // }

  let width = matrix[0].len();
  let height = matrix.len();

  let mut cache: HashMap<String, i32> = HashMap::new();
  let mut i = 0;
  let mut state = matrix.iter().flat_map(|row| row.iter()).collect::<String>();
  while !cache.contains_key(&state) {
    cache.insert(state.clone(), i);
    move_north(&mut matrix, width, height);
    move_west(&mut matrix, width, height);
    move_south(&mut matrix, width, height);
    move_east(&mut matrix, width, height);
    i += 1;
    state = matrix.iter().flat_map(|row| row.iter()).collect::<String>();
  }

  let cycle = i - cache.get(&state).unwrap();
  let remaining = 1000000000 - i;
  for _ in 0..remaining % cycle {
    move_north(&mut matrix, width, height);
    move_west(&mut matrix, width, height);
    move_south(&mut matrix, width, height);
    move_east(&mut matrix, width, height);
  }

  // println!("\nFinal matrix:");
  // for row in matrix.iter() {
  //   println!("{}", row.iter().collect::<String>());
  // }

  let mut load = 0;
  for (r, row) in matrix.iter().enumerate() {
    for c in row.iter() {
      if *c == 'O' {
        load += height - r;
      }
    }
  }

  println!("Load: {}", load);
}
