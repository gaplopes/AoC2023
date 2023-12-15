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

  let mut adj_list: Vec<Vec<i32>> = vec![vec![]; width * height + 1];
  let mut start_point: (i32, i32) = (0, 0);

  for i in 0..height {
    for j in 0..width {
      match matrix[i].chars().nth(j).unwrap() {
        'S' => {
          start_point = (i as i32, j as i32);
        }
        '.' => {}
        _ => {
          let tile = tiles
            .get(&matrix[i].chars().nth(j).unwrap())
            .expect("Invalid tile");
          for &(di, dj) in tile {
            let x = i as i32 + di;
            let y = j as i32 + dj;
            if !can_connect(x as usize, y as usize, &matrix) {
              continue;
            }
            let other_tile = tiles
              .get(&matrix[x as usize].chars().nth(y as usize).unwrap())
              .expect("Invalid tile");
            for &(odi, odj) in other_tile {
              if odi + di == 0 && odj + dj == 0 {
                // adj_list[i * width + j].push(x * width as i32 + y);
                adj_list[x as usize * width + y as usize].push(i as i32 * width as i32 + j as i32);
                break;
              }
            }
          }
        }
      }
    }
  }

  println!("Matrix:");
  for row in &matrix {
    println!("{}", row);
  }

  println!("Start point: {:?}", start_point);
  println!("Adjacency list:");
  for (i, adj) in adj_list
    .iter()
    .enumerate()
    .filter(|(_, adj)| !adj.is_empty())
  {
    println!(
      "({}, {}): {:?}",
      i / width,
      i % width,
      adj
        .iter()
        .map(|&v| (v / width as i32, v % width as i32))
        .collect::<Vec<(i32, i32)>>()
    );
  }

  // Compute distances using BFS
  let mut dist: Vec<i64> = vec![std::i64::MAX; width * height + 1];
  let mut heap = BinaryHeap::new();
  heap.push(Reverse((0, start_point)));
  while let Some(Reverse((d, (i, j)))) = heap.pop() {
    if dist[i as usize * width + j as usize] == std::i64::MAX {
      dist[i as usize * width + j as usize] = d;
      for &v in &adj_list[i as usize * width + j as usize] {
        heap.push(Reverse((d + 1, (v / width as i32, v % width as i32))));
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
