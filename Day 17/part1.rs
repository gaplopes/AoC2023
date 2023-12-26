use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::Read;

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  // Create a grid of numbers from the input
  let mut grid: Vec<Vec<i32>> = Vec::new();
  for line in input.lines() {
    let mut row: Vec<i32> = Vec::new();
    for c in line.chars() {
      let n: i32 = c.to_digit(10).expect("Not a digit") as i32;
      row.push(n);
    }
    grid.push(row);
  }

  let width = grid[0].len();
  let height = grid.len();

  let dirs: HashMap<char, (i32, i32)> =
    [('U', (-1, 0)), ('R', (0, 1)), ('D', (1, 0)), ('L', (0, -1)), ('S', (0, 0))]
      .iter()
      .cloned()
      .collect();
  let mut heap: BinaryHeap<(Reverse<i32>, ((i32, i32), (i32, i32), String))> = BinaryHeap::new();
  let mut visited: HashMap<((i32, i32), (i32, i32)), bool> = HashMap::new();
  let start = (0, 0);
  let end = (height as i32 - 1, width as i32 - 1);
  heap.push((Reverse(0), (start, (0, 0), String::new())));
  while let Some((Reverse(d), (pos, dir, path))) = heap.pop() {
    if pos == end {
      println!("Found end with dist {} and path {}", d, path);
      break;
    }
    if visited.contains_key(&(pos, dir)) {
      continue;
    }
    visited.insert((pos, dir), true);
    let (i, j) = pos;
    for (c, (di, dj)) in dirs.iter() {
      if (*di, *dj) == (-dir.0, -dir.1) || (*di, *dj) == dir || (*di, *dj) == (0, 0) {
        continue;
      }
      let mut npos = (i, j);
      let mut nd = d;
      let mut npath = path.clone();
      for _ in 0..3 {
        npos = (npos.0 + di, npos.1 + dj);
        if npos.0 >= height as i32 || npos.1 >= width as i32 || npos.0 < 0 || npos.1 < 0 {
          break;
        }
        nd += grid[npos.0 as usize][npos.1 as usize];
        npath.push(*c);
        heap.push((Reverse(nd), (npos, (*di, *dj), npath.clone())));
      }
    }
  }
}
