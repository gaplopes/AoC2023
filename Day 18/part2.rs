use std::io::Read;
use std::collections::HashMap;

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  // Split each line, eg: R 6 (#70c710) -> ["R", 6]
  let dirs_id: HashMap<i32, &str> = [
    (0, "R"),
    (1, "D"),
    (2, "L"),
    (3, "U"),
  ].iter().cloned().collect();

  let mut moves: Vec<(&str, i64)> = Vec::new();
  for line in input.lines() {
    let mut parts = line.split_whitespace();
    let _dir = parts.next().unwrap();
    let _dist = parts.next().unwrap().parse::<i32>().unwrap();
    let _hexadecimal = parts.next().unwrap();
    // Get last character of _hexadecimal
    let last_char = &_hexadecimal[_hexadecimal.len() - 2.._hexadecimal.len() - 1];
    let dir = dirs_id[&last_char.parse::<i32>().unwrap()];
    // Remove first and last characters of hexadecimal
    let hexadecimal: &str = &_hexadecimal[2.._hexadecimal.len() - 2];
    // Convert hexadecimal to decimal
    let dist = i64::from_str_radix(hexadecimal, 16).unwrap();
    moves.push((dir, dist));
    // println!("{} {}", dir, dist);
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

  let _height = max_x - min_x + 1;
  let _width = max_y - min_y + 1;

  // Start at top left
  let mut pos = (min_x.abs() as i64, min_y.abs() as i64);
  let mut points: Vec<(i64, i64)> = Vec::new();
  for (dir, dist) in &moves {
    match *dir {
      "D" => {
        pos = (pos.0 + *dist as i64, pos.1);
      }
      "U" => {
        pos = (pos.0 - *dist as i64, pos.1);
      }
      "R" => {
        pos = (pos.0, pos.1 + *dist as i64);
      }
      "L" => {
        pos = (pos.0, pos.1 - *dist as i64);
      }
      _ => panic!("Unknown direction: {}", dir),
    }
    points.push(pos);
    // println!("{} {}", pos.0, pos.1);
  }

  // Area of polygon: Shoelace formula
  // https://en.wikipedia.org/wiki/Shoelace_formula
  let mut area: f64 = 0.0;
  for i in 0..points.len() {
    let q = points[i];
    let p = points[(i + 1) % points.len()];
    area += (p.0 as f64 - q.0 as f64) * (p.1 as f64 + q.1 as f64);
  }
  area = area.abs() / 2.0;
  println!("Area: {}", area);

  // Perimeter of polygon
  let mut perimeter: f64 = 0.0;
  for i in 0..points.len() {
    let q = points[i];
    let p = points[(i + 1) % points.len()];
    perimeter += ((p.0 as f64 - q.0 as f64).powi(2) + (p.1 as f64 - q.1 as f64).powi(2)).sqrt();
  }
  println!("Perimeter: {}", perimeter);

  // Area of polygon: Pick's theorem
  // https://en.wikipedia.org/wiki/Pick%27s_theorem
  let interior_points = area - perimeter / 2.0 + 1.0;
  println!("Interior points: {}", interior_points);

  // Filled area of the polygon (including perimeter)
  let filled_area = interior_points + perimeter;
  println!("Filled area: {}", filled_area);

  // https://www.quora.com/Is-there-any-mathematical-algorithm-to-find-the-area-of-any-shape-using-boundary-coordinates
}
