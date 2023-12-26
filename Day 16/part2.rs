use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::io::Read;

fn can_move(i: i32, j: i32, grid: &Vec<Vec<char>>) -> bool {
  // println!("Testing {},{} with {} and {}", i, j, grid.len(), grid[0].len());
  !(i >= grid.len() as i32 || j >= grid[0].len() as i32 || i < 0 || j < 0)
}

fn get_positions(
  i: i32,
  j: i32,
  d: &char,
  dirs: &HashMap<char, (i32, i32)>,
  grid: &Vec<Vec<char>>,
  marked_grid: &mut Vec<Vec<char>>,
) -> Vec<(i32, i32, char)> {
  let mut pos: Vec<(i32, i32, char)> = Vec::new();
  let current_char: char = grid[i as usize][j as usize];
  marked_grid[i as usize][j as usize] = '#';
  // println!("Current char {}", current_char);
  if current_char == '/' || current_char == '\\' {
    let moves: HashMap<char, (i32, i32)> =
      [('L', (1, 0)), ('R', (-1, 0)), ('U', (0, 1)), ('D', (0, -1))]
        .iter()
        .cloned()
        .collect();
    let d_first: HashMap<char, char> = [('L', 'D'), ('R', 'U'), ('U', 'R'), ('D', 'L')].iter().cloned().collect();
    let d_second: HashMap<char, char> = [('L', 'U'), ('R', 'D'), ('U', 'L'), ('D', 'R')].iter().cloned().collect();
    let next_pos: (i32, i32, char) = (
      i as i32 + moves[d].0 * (if current_char == '/' {1} else {-1}),
      j as i32 + moves[d].1 * (if current_char == '/' {1} else {-1}),
      if current_char == '/' {d_first[d]} else {d_second[d]},
    );
    pos.push(next_pos);
  } else if current_char == '-' {
    if *d == 'L' || *d == 'R' {
      let next_pos: (i32, i32, char) = (
        i as i32 + dirs[d].0,
        j as i32 + dirs[d].1,
        *d,
      );
      pos.push(next_pos);
    } else {
      let next_pos_one: (i32, i32, char) = (
        i as i32,
        j as i32 + 1,
        'R',
      );
      pos.push(next_pos_one);
      let next_pos_two: (i32, i32, char) = (
        i as i32,
        j as i32 - 1,
        'L',
      );
      pos.push(next_pos_two);
    }
  } else if current_char == '|' {
    if *d == 'U' || *d == 'D' {
      let next_pos: (i32, i32, char) = (
        i as i32 + dirs[d].0,
        j as i32 + dirs[d].1,
        *d,
      );
      pos.push(next_pos);
    } else {
      let next_pos_one: (i32, i32, char) = (
        i as i32 - 1,
        j as i32,
        'U',
      );
      pos.push(next_pos_one);
      let next_pos_two: (i32, i32, char) = (
        i as i32 + 1,
        j as i32,
        'D',
      );
      pos.push(next_pos_two);
    }
  } else {
    let next_pos: (i32, i32, char) = (i as i32 + dirs[d].0, j as i32 + dirs[d].1, *d);
    pos.push(next_pos);
  }
  return pos;
}

fn energize_tiles(start: (i32, i32), dir: char, grid: &Vec<Vec<char>>) -> i32 {
  let dirs: HashMap<char, (i32, i32)> =
    [('L', (0, -1)), ('R', (0, 1)), ('U', (-1, 0)), ('D', (1, 0))]
    .iter()
    .cloned()
    .collect();
  let mut marked_grid: Vec<Vec<char>> = grid.clone();
  let mut cache_moves: HashMap<(i32, i32, char), bool> = HashMap::new();
  let aux_start: (i32, i32, char) = (start.0, start.1, dir);
  let mut heap = BinaryHeap::new();
  heap.push(aux_start);
  while let Some((i, j, d)) = heap.pop() {
    if !can_move(i, j, &grid) || cache_moves.contains_key(&(i, j, d)) {
      continue;
    }
    // println!("- At position {},{} with dir {}", i, j, d);
    cache_moves.insert((i, j, d), true);
    let next_positions: Vec<(i32, i32, char)> = get_positions(i, j, &d, &dirs, &grid, &mut marked_grid);
    for next_position in &next_positions {
      heap.push(*next_position);
      // println!("  Adding {:?}", next_position);
    }
    // println!("Grid marked:");
    // for row in &marked_grid {
    //   let string: String = row.into_iter().collect();
    //   println!("{}", string);
    // }
  }

  let mut count: i32 = 0;
  // println!("Grid marked:");
  for row in &marked_grid {
    // let string: String = row.into_iter().collect();
    // println!("{}", string);
    // Count # in string
    count += row.iter().filter(|&c| *c == '#').count() as i32;
  }

  // println!("Count: {}", count);
  return count;
}

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

  // TODO: I think this can be done with DP (memoization) instead of brute force
  
  let mut best_count: i32 = 0;
  // Any tile of the top row (that is a dot) starts with dir 'D'
  for j in 0..grid[0].len() {
    if grid[0][j] == '.' {
      let count: i32 = energize_tiles((0, j as i32), 'D', &grid);
      if count > best_count {
        best_count = count;
      }
    }
  }
  // Any tile of the bottom row (that is a dot) starts with dir 'U'
  for j in 0..grid[0].len() {
    if grid[grid.len() - 1][j] == '.' {
      let count: i32 = energize_tiles((grid.len() as i32 - 1, j as i32), 'U', &grid);
      if count > best_count {
        best_count = count;
      }
    }
  }
  // Any tile of the left column (that is a dot) starts with dir 'R'
  for i in 0..grid.len() {
    if grid[i][0] == '.' {
      let count: i32 = energize_tiles((i as i32, 0), 'R', &grid);
      if count > best_count {
        best_count = count;
      }
    }
  }
  // Any tile of the right column (that is a dot) starts with dir 'L'
  for i in 0..grid.len() {
    if grid[i][grid[0].len() - 1] == '.' {
      let count: i32 = energize_tiles((i as i32, grid[0].len() as i32 - 1), 'L', &grid);
      if count > best_count {
        best_count = count;
      }
    }
  }
  println!("Best count: {}", best_count);
}
