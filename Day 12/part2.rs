use std::io::Read;
use std::collections::HashMap;

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  let mut sum: i64 = 0;
  for line in input.lines() {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let aux_field = parts[0];
    let aux_pipes: Vec<i32> = parts[1].split(',').map(|x| x.parse::<i32>().unwrap()).collect();

    // Copy the field 5 times and add ? between them
    let mut other_field = String::new();
    other_field.push_str(aux_field);
    for _ in 0..4 {
      other_field.push('?');
      other_field.push_str(aux_field);
    }
    let field = other_field.as_str();
    // Copy the pipes 5 times
    let mut pipes: Vec<i32> = Vec::new();
    for _ in 0..5 {
      pipes.extend(&aux_pipes);
    }

    // println!("{} {:?}", field, pipes);
    
    let mut cache: HashMap<String, i64> = HashMap::new();

    fn recursive(field: &str, pipes: &Vec<i32>, field_idx: usize, pipe_idx: usize, field_cpy: &mut String, cache: &mut HashMap<String, i64>) -> i64 {
      // println!("- {} {}", field_idx, pipe_idx);
      let key = format!("{} {}", field_idx, pipe_idx);
      if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
      }
      
      if field_idx >= field.len() {
        if pipe_idx == pipes.len() {
          // println!("Found one {}", field_cpy);
          return 1;
        }
        return 0;
      }

      if pipe_idx >= pipes.len() {
        let count_chars = field[field_idx..].chars().filter(|x| *x == '#').count();
        if count_chars == 0 {
          // println!("Found one {}", field_cpy);
          return 1;
        }
        return 0;
      }

      let mut sum: i64 = 0;
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
          sum += recursive(field, pipes, field_idx + length as usize + 1, pipe_idx + 1, field_cpy, cache);
          field_cpy.replace_range(field_idx..field_idx + length as usize, &field[field_idx..field_idx + length as usize]);
          if field_idx + (length as usize) < field.len() {
            field_cpy.replace_range(field_idx + length as usize..field_idx + length as usize + 1, &field[field_idx + length as usize..field_idx + length as usize + 1]);
          }
        }
      }
      if field.chars().nth(field_idx).unwrap() == '.' || field.chars().nth(field_idx).unwrap() == '?' {
        field_cpy.replace_range(field_idx..field_idx + 1, &".");
        sum += recursive(field, pipes, field_idx + 1, pipe_idx, field_cpy, cache);
        field_cpy.replace_range(field_idx..field_idx + 1, &field[field_idx..field_idx + 1]);
      }
      cache.insert(key, sum);
      return sum;
    }

    let mut field_cpy = field.to_string();
    let count = recursive(field, &pipes, 0, 0, &mut field_cpy, &mut cache);
    // println!("{}", count);
    sum += count;
  }
  println!("{}", sum);
}
