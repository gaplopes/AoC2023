use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::Read;

fn can_connect(i: usize, j: usize, matrix: &Vec<Vec<char>>) -> bool {
  !(i >= matrix.len() || j >= matrix[0].len())
}

fn tiles_connect_horizontal(tile: &Vec<&str>, other_tile: &Vec<&str>) -> bool {
  if tile.contains(&"east") && other_tile.contains(&"west") {
    return true;
  }
  return false;
}

fn tiles_connect_vertical(tile: &Vec<&str>, other_tile: &Vec<&str>) -> bool {
  if tile.contains(&"south") && other_tile.contains(&"north") {
    return true;
  }
  return false;
}

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let matrix: Vec<&str> = input.lines().collect();
  let width = matrix[0].len();
  let height = matrix.len();

  let tiles: HashMap<char, Vec<&str>> = [
    ('L', vec!["north", "east"]),
    ('J', vec!["north", "west"]),
    ('7', vec!["south", "west"]),
    ('F', vec!["south", "east"]),
    ('|', vec!["north", "south"]),
    ('-', vec!["east", "west"]),
    ('S', vec!["north", "south", "east", "west"]),
  ]
  .iter()
  .cloned()
  .collect();

  // println!("{} {}", width, height);
  // println!("Matrix:");
  // for i in 0..height {
  //   println!("{}", matrix[i]);
  // }

  let mut new_matrix_columns: Vec<Vec<char>> = vec![vec![]; height];

  for i in 0..width - 1 {
    if i % 2 == 0 {
      for j in 0..height {
        new_matrix_columns[j].push(matrix[j].chars().nth(i).expect("Invalid matrix"));
      }
    }

    for j in 0..height {
      let prev_tile = matrix[j].chars().nth(i).expect("Invalid matrix");
      let next_tile = matrix[j].chars().nth(i + 1).expect("Invalid matrix");
      if prev_tile == '.' || next_tile == '.' {
        new_matrix_columns[j].push('*');
      } else {
        let prev_tile_vec = tiles.get(&prev_tile).expect("Invalid tile");
        let next_tile_vec = tiles.get(&next_tile).expect("Invalid tile");
        if tiles_connect_horizontal(prev_tile_vec, next_tile_vec) {
          new_matrix_columns[j].push('-');
        } else {
          new_matrix_columns[j].push('*');
        }
      }
    }

    if i % 2 == 0 {
      for j in 0..height {
        new_matrix_columns[j].push(matrix[j].chars().nth(i + 1).expect("Invalid matrix"));
      }
    }
  }

  // // Print new matrix
  // println!("New matrix X:");
  // for i in 0..height {
  //   println!("{}", new_matrix_columns[i].iter().collect::<String>());
  // }

  let mut new_matrix_lines: Vec<Vec<char>> = vec![];
  let new_width = new_matrix_columns[0].len();

  for i in 0..height - 1 {
    if i % 2 == 0 {
      new_matrix_lines.push(new_matrix_columns[i].clone());
    }

    let mut new_matrix_line: Vec<char> = vec![];
    for j in 0..new_width {
      let prev_tile = new_matrix_columns[i][j];
      let next_tile = new_matrix_columns[i + 1][j];
      if prev_tile == '.' || next_tile == '.' || prev_tile == '*' || next_tile == '*' {
        new_matrix_line.push('*');
      } else {
        let prev_tile_vec = tiles.get(&prev_tile).expect("Invalid tile");
        let next_tile_vec = tiles.get(&next_tile).expect("Invalid tile");
        if tiles_connect_vertical(prev_tile_vec, next_tile_vec) {
          new_matrix_line.push('|');
        } else {
          new_matrix_line.push('*');
        }
      }
    }
    new_matrix_lines.push(new_matrix_line);

    if i % 2 == 0 {
      new_matrix_lines.push(new_matrix_columns[i + 1].clone());
    }
  }

  let new_height = new_matrix_lines.len();

  // // Print new matrix
  // println!("New matrix Y:");
  // for i in 0..new_matrix_lines.len() {
  //   println!("{}", new_matrix_lines[i].iter().collect::<String>());
  // }

  // BFS from the start point (S in the new matrix lines)
  let mut start_point = (0, 0);
  for i in 0..new_height {
    for j in 0..new_width {
      if new_matrix_lines[i][j] == 'S' {
        start_point.0 = i;
        start_point.1 = j;
        break;
      }
    }
  }

  let mut dist: Vec<i64> = vec![std::i64::MAX; new_width * new_height + 1];
  let mut heap = BinaryHeap::new();
  heap.push(Reverse((0, start_point)));
  while let Some(Reverse((d, (i, j)))) = heap.pop() {
    if dist[i * new_width + j] != std::i64::MAX {
      continue;
    }
    dist[i * new_width + j] = d;
    let dirs: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (di, dj) in dirs {
      let ni: i32 = i as i32 + di;
      let nj: i32 = j as i32 + dj;
      if can_connect(ni as usize, nj as usize, &new_matrix_lines) {
        // Verify if its . or *
        if vec!['.', '*', 'O'].contains(&new_matrix_lines[ni as usize][nj as usize]) {
          continue;
        }
        let actual_tile = tiles
          .get(&new_matrix_lines[i as usize][j as usize])
          .expect("Invalid tile");
        let next_tile = tiles
          .get(&new_matrix_lines[ni as usize][nj as usize])
          .expect("Invalid tile");
        match (di, dj) {
          (-1, 0) => {
            if tiles_connect_vertical(next_tile, actual_tile) {
              heap.push(Reverse((d + 1, (ni as usize, nj as usize))));
            }
          }
          (1, 0) => {
            if tiles_connect_vertical(actual_tile, next_tile) {
              heap.push(Reverse((d + 1, (ni as usize, nj as usize))));
            }
          }
          (0, -1) => {
            if tiles_connect_horizontal(next_tile, actual_tile) {
              heap.push(Reverse((d + 1, (ni as usize, nj as usize))));
            }
          }
          (0, 1) => {
            if tiles_connect_horizontal(actual_tile, next_tile) {
              heap.push(Reverse((d + 1, (ni as usize, nj as usize))));
            }
          }
          _ => {}
        }
      }
    }

    new_matrix_lines[i][j] = 'O';
  }

  // Flood fill from the border
  let mut visited: Vec<Vec<bool>> = vec![vec![false; new_width]; new_height];
  let mut queue: BinaryHeap<(Reverse<i32>, i32, i32)> = BinaryHeap::new();

  // Add each border cell to the queue
  for i in 0..new_height {
    for j in 0..new_width {
      if i == 0 || i == new_height - 1 || j == 0 || j == new_width - 1 {
        if new_matrix_lines[i][j] == '.' || new_matrix_lines[i][j] == '*' {
          queue.push((Reverse(0), i as i32, j as i32));
        }
      }
    }
  }

  while !queue.is_empty() {
    let (Reverse(dist), i, j) = queue.pop().unwrap();
    if visited[i as usize][j as usize] {
      continue;
    }
    visited[i as usize][j as usize] = true;
    new_matrix_lines[i as usize][j as usize] = '*';

    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (di, dj) in dirs {
      let ni = i + di;
      let nj = j + dj;
      if can_connect(ni as usize, nj as usize, &new_matrix_lines) {
        if new_matrix_lines[ni as usize][nj as usize] != 'O' {
          queue.push((Reverse(dist + 1), ni, nj));
        }
      }
    }
  }

  // // Print new matrix
  // println!("Final matrix:");
  // for i in 0..new_matrix_lines.len() {
  //   println!("{}", new_matrix_lines[i].iter().collect::<String>());
  // }

  // Count the number of diff cells that lay in odd columns or rows
  let mut diff_count = 0;
  for i in 0..new_height {
    if i % 2 != 0 {
      continue;
    }
    for j in 0..new_width {
      if j % 2 != 0 {
        continue;
      }
      if new_matrix_lines[i][j] != '*' && new_matrix_lines[i][j] != 'O' {
        diff_count += 1;
      }
    }
  }
  println!("{}", diff_count);
}
