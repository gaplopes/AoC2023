use std::io::Read;

fn find_point(grid: &Vec<Vec<char>>) -> (i32, i32) {
  // Find a point that is a dot and has a dot to the left and right
  let mut x = 0;
  for row in grid {
    let mut y = 0;
    for c in row {
      if *c == '.' {
        let mut left = false;
        let mut right = false;
        let aux_row = x;
        let mut aux_col = y;
        while aux_col > 0 {
          aux_col -= 1;
          if grid[aux_row][aux_col] == '#' {
            left = true;
            break;
          }
        }
        aux_col = y;
        while aux_col < row.len() - 1 {
          aux_col += 1;
          if grid[aux_row][aux_col] == '#' {
            right = true;
            break;
          }
        }
        if left && right {
          return (x as i32, y as i32);
        }
      }
      y += 1;
    }
    x += 1;
  }
  panic!("No point found");
}

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  // Split each line, eg: R 6 (#70c710) -> ["R", 6]
  let mut moves: Vec<(&str, i32)> = Vec::new();
  for line in input.lines() {
    let mut parts = line.split_whitespace();
    let dir = parts.next().unwrap();
    let dist = parts.next().unwrap().parse::<i32>().unwrap();
    moves.push((dir, dist));
  }

  // Maximum height and width of the grid
  let mut max_x = 0;
  let mut min_x = 0;
  let mut max_y = 0;
  let mut min_y = 0;
  let mut pos = (0, 0);
  for (dir, dist) in &moves {
    match *dir {
      "D" => pos.0 += dist,
      "U" => pos.0 -= dist,
      "R" => pos.1 += dist,
      "L" => pos.1 -= dist,
      _ => panic!("Unknown direction: {}", dir),
    }
    max_x = std::cmp::max(max_x, pos.0);
    min_x = std::cmp::min(min_x, pos.0);
    max_y = std::cmp::max(max_y, pos.1);
    min_y = std::cmp::min(min_y, pos.1);
  }

  let height = max_x - min_x + 1;
  let width = max_y - min_y + 1;

  // Create a grid of the appropriate size
  let mut grid: Vec<Vec<char>> = Vec::new();
  for _ in 0..height {
    let mut row: Vec<char> = Vec::new();
    for _ in 0..width {
      row.push('.');
    }
    grid.push(row);
  }

  // println!("{} {}", grid.len(), grid[0].len());

  // Start at top left
  let mut pos = (min_x.abs(), min_y.abs());
  for m in moves {
    let (dir, dist) = m;
    let (x, y) = pos;
    // println!("At {} {} moving {} {}", x, y, dir, dist);
    match dir {
      "R" => {
        for i in 0..dist {
          grid[x as usize][(y as i32 + i) as usize] = '#';
        }
        pos = (x, y + dist);
      }
      "L" => {
        for i in 0..dist {
          grid[x as usize][(y as i32 - i) as usize] = '#';
        }
        pos = (x, y - dist);
      }
      "U" => {
        for i in 0..dist {
          grid[(x as i32 - i) as usize][y as usize] = '#';
        }
        pos = (x - dist, y);
      }
      "D" => {
        for i in 0..dist {
          grid[(x as i32 + i) as usize][y as usize] = '#';
        }
        pos = (x + dist, y);
      }
      _ => panic!("Unknown direction: {}", dir),
    }
  }

  // // Print the grid
  // for row in &grid {
  //   for c in row {
  //     print!("{}", c);
  //   }
  //   println!();
  // }

  let (x, y) = find_point(&grid);
  // println!("Point inside: {} {}", x, y);

  // Start flood fill
  let mut stack: Vec<(i32, i32)> = Vec::new();
  stack.push((x, y));
  while !stack.is_empty() {
    let (x, y) = stack.pop().unwrap();
    if grid[x as usize][y as usize] == '.' {
      grid[x as usize][y as usize] = '#';
      stack.push((x - 1, y));
      stack.push((x + 1, y));
      stack.push((x, y - 1));
      stack.push((x, y + 1));
    }
  }

  // // Print the grid
  // for row in &grid {
  //   for c in row {
  //     print!("{}", c);
  //   }
  //   println!();
  // }

  // Count the number of #
  let mut count = 0;
  for row in &grid {
    for c in row {
      if *c == '#' {
        count += 1;
      }
    }
  }

  println!("Count: {}", count);
}
