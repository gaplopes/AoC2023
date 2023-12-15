use std::io::Read;

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let mut sum: i64 = 0;
  for line in input.lines() {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let field = parts[0];
    let pipes: Vec<i32> = parts[1].split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    // println!("{} {:?}", field, pipes);
    let mut count: i64 = 0;

    fn recursive(field: &str, pipes: &Vec<i32>, count: &mut i64, field_idx: usize, pipe_idx: usize, field_cpy: &mut String) {
      // println!("- {} {}", field_idx, pipe_idx);
      if field_idx >= field.len() {
        if pipe_idx == pipes.len() {
          // println!("Found one {}", field_cpy);
          *count += 1;
        }
        return;
      }

      if pipe_idx >= pipes.len() {
        let count_chars = field[field_idx..].chars().filter(|x| *x == '#').count();
        if count_chars == 0 {
          // println!("Found one {}", field_cpy);
          *count += 1;
        }
        return;
      }

      if field.chars().nth(field_idx).unwrap() == '#' || field.chars().nth(field_idx).unwrap() == '?' {
        let mut can_place: bool = true;
        let length = pipes[pipe_idx];
        if field_idx + length as usize > field.len() {
          // println!("Not enough space");
          can_place = false;
        }
        if field_idx + (length as usize) < field.len() && field.chars().nth(field_idx + length as usize).unwrap() == '#' {
          // println!("Overlapping");
          can_place = false;
        }
        
        if can_place == true {
          let count_chars = field[field_idx..field_idx + length as usize].chars().filter(|x| *x == '.').count();
          if count_chars != 0 {
            // println!("Dot in the middle");
            can_place = false;
          }
        }
        if can_place {
          field_cpy.replace_range(field_idx..field_idx + length as usize, &"#".repeat(length as usize));
          if field_idx + (length as usize) < field.len() {
            field_cpy.replace_range(field_idx + length as usize..field_idx + length as usize + 1, &".");
          }
          recursive(field, pipes, count, field_idx + length as usize + 1, pipe_idx + 1, field_cpy);
          field_cpy.replace_range(field_idx..field_idx + length as usize, &field[field_idx..field_idx + length as usize]);
          if field_idx + (length as usize) < field.len() {
            field_cpy.replace_range(field_idx + length as usize..field_idx + length as usize + 1, &field[field_idx + length as usize..field_idx + length as usize + 1]);
          }
        }
      }
      if field.chars().nth(field_idx).unwrap() == '.' || field.chars().nth(field_idx).unwrap() == '?' {
        field_cpy.replace_range(field_idx..field_idx + 1, &".");
        recursive(field, pipes, count, field_idx + 1, pipe_idx, field_cpy);
        field_cpy.replace_range(field_idx..field_idx + 1, &field[field_idx..field_idx + 1]);
      }
    }

    let mut field_cpy = field.to_string();
    recursive(field, &pipes, &mut count, 0, 0, &mut field_cpy);
    // println!("{}", count);
    sum += count;
  }
  println!("{}", sum);
}
