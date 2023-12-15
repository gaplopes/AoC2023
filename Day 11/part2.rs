use std::cmp::Ordering;
use std::io::Read;

fn compute_dist(
  (mut x1, mut y1): (usize, usize),
  (x2, y2): (usize, usize),
  empty_rows: &Vec<bool>,
  empty_columns: &Vec<bool>,
  offset: i64,
) -> i64 {
  let mut dist: i64 = 0;

  while x1 != x2 {
    dist += if empty_rows[x1] { offset } else { 1 };
    x1 = match x1.cmp(&x2) {
      Ordering::Less => x1 + 1,
      Ordering::Greater => x1 - 1,
      _ => x1,
    };
  }

  while y1 != y2 {
    dist += if empty_columns[y1] { offset } else { 1 };
    y1 = match y1.cmp(&y2) {
      Ordering::Less => y1 + 1,
      Ordering::Greater => y1 - 1,
      _ => y1,
    };
  }

  return dist;
}

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let rows = matrix.clone();
  let columns: Vec<Vec<char>> = (0..matrix[0].len())
    .map(|i| matrix.iter().map(|row| row[i]).collect())
    .collect();

  // Create vector of boolean values to indicate if a row or column is empty
  let mut empty_rows: Vec<bool> = vec![false; matrix.len()];
  let mut empty_columns: Vec<bool> = vec![false; matrix[0].len()];
  for (i, row) in rows.iter().enumerate() {
    let r = row.iter().filter(|&&ch| ch == '#').count();
    if r == 0 {
      empty_rows[i] = true;
    }
  }
  for (i, column) in columns.iter().enumerate() {
    let c = column.iter().filter(|&&ch| ch == '#').count();
    if c == 0 {
      empty_columns[i] = true;
    }
  }

  let mut galaxies: Vec<(i32, (usize, usize))> = vec![];
  let mut count = 1;
  for (i, row) in matrix.iter().enumerate() {
    for (j, ch) in row.iter().enumerate() {
      if *ch == '#' {
        galaxies.push((count, (i, j)));
        count += 1;
      }
    }
  }

  let n = galaxies.len();
  let offset: i64 = 1000000;

  // Floyd Warshall algorithm to find shortest path between all pairs of vertices
  let mut dist = vec![vec![std::i64::MAX; n]; n];
  for i in 0..n {
    for j in 0..n {
      if i == j {
        dist[i][j] = 0;
      } else {
        dist[i][j] = compute_dist(
          galaxies[i].1,
          galaxies[j].1,
          &empty_rows,
          &empty_columns,
          offset,
        );
      }
    }
  }

  for k in 0..n {
    for i in 0..n {
      for j in 0..n {
        if dist[i][k] != std::i64::MAX
          && dist[k][j] != std::i64::MAX
          && dist[i][k] + dist[k][j] < dist[i][j]
        {
          dist[i][j] = dist[i][k] + dist[k][j];
        }
      }
    }
  }

  // Sum all distances between all pairs of vertices
  let mut sum = 0;
  for i in 0..n {
    for j in i + 1..n {
      sum += dist[i][j];
    }
  }

  println!("{}", sum);
}
