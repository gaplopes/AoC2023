use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::Read;

fn can_connect(i: usize, j: usize, matrix: &Vec<&str>) -> bool {
  !(i >= matrix.len() || j >= matrix[0].len() || matrix[i].chars().nth(j).unwrap() == '.')
}

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let matrix: Vec<&str> = input.lines().collect();
  let width = matrix[0].len();
  let height = matrix.len();

  let tiles: HashMap<char, Vec<(i32, i32)>> = [
    ('L', vec![(-1, 0), (0, 1)]),
    ('J', vec![(-1, 0), (0, -1)]),
    ('7', vec![(1, 0), (0, -1)]),
    ('F', vec![(1, 0), (0, 1)]),
    ('|', vec![(-1, 0), (1, 0)]),
    ('-', vec![(0, -1), (0, 1)]),
    ('S', vec![(1, 0), (0, 1), (-1, 0), (0, -1)]),
  ]
  .iter()
  .cloned()
  .collect();

  // Find start point
  let mut start_point = (0, 0);
  for i in 0..height {
    for j in 0..width {
      if matrix[i].chars().nth(j).unwrap() == 'S' {
        start_point = (i as i32, j as i32);
        break;
      }
    }
  }

  // Compute distances using BFS
  let mut dist: Vec<i64> = vec![std::i64::MAX; width * height + 1];
  let mut heap = BinaryHeap::new();
  heap.push(Reverse((0, start_point)));
  while let Some(Reverse((d, (i, j)))) = heap.pop() {
    if dist[i as usize * width + j as usize] == std::i64::MAX {
      dist[i as usize * width + j as usize] = d;
      for (di, dj) in tiles[&matrix[i as usize].chars().nth(j as usize).unwrap()].iter() {
        let (ni, nj) = (i + di, j + dj);
        if can_connect(ni as usize, nj as usize, &matrix) {
          for (odi, odj) in tiles[&matrix[ni as usize].chars().nth(nj as usize).unwrap()].iter() {
            if di + odi == 0 && dj + odj == 0 {
              heap.push(Reverse((d + 1, (ni, nj))));
            }
          }
        }
      }
    }
  }

  println!("Distances:");
  for i in 0..height {
    for j in 0..width {
      print!(
        "{:2} ",
        if dist[i * width + j] == std::i64::MAX {
          "0".to_string()
        } else {
          dist[i * width + j].to_string()
        }
      );
    }
    println!();
  }

  println!(
    "Max distance: {}",
    dist
      .iter()
      .filter(|&&d| d != std::i64::MAX)
      .max()
      .expect("No max distance found")
  );
}
