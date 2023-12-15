use std::io::Read;

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let rows = matrix.clone();
  let columns: Vec<Vec<char>> = (0..matrix[0].len())
    .map(|i| matrix.iter().map(|row| row[i]).collect())
    .collect();

  let mut new_matrix: Vec<Vec<char>> = vec![];
  for row in rows {
    let mut new_row: Vec<char> = vec![];
    for (i, ch) in row.iter().enumerate() {
      let c = columns[i].iter().filter(|&&ch| ch == '#').count();
      new_row.push(*ch);
      if c == 0 {
        new_row.push('.');
      }
    }
    let r = row.iter().filter(|&&ch| ch == '#').count();
    new_matrix.push(new_row.clone());
    if r == 0 {
      new_matrix.push(new_row.clone());
    }
  }

  // for row in new_matrix {
  //   println!("{}", row.iter().collect::<String>());
  // }

  let mut galaxies: Vec<(i32, (usize, usize))> = vec![];
  let mut count = 1;
  for (i, row) in new_matrix.iter().enumerate() {
    for (j, ch) in row.iter().enumerate() {
      if *ch == '#' {
        galaxies.push((count, (i, j)));
        count += 1;
      }
    }
  }

  let n = galaxies.len();

  // Manhattan distance between two points
  let compute_dist = |(x1, y1): (usize, usize), (x2, y2): (usize, usize)| {
    ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as i32
  };

  // Floyd Warshall algorithm to find shortest path between all pairs of vertices
  let mut dist = vec![vec![std::i32::MAX; n]; n];
  for i in 0..n {
    for j in 0..n {
      if i == j {
        dist[i][j] = 0;
      } else {
        dist[i][j] = compute_dist(galaxies[i].1, galaxies[j].1);
      }
    }
  }

  for k in 0..n {
    for i in 0..n {
      for j in 0..n {
        if dist[i][k] != std::i32::MAX
          && dist[k][j] != std::i32::MAX
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
