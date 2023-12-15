use std::collections::HashMap;
use std::io::Read;

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  // Read first line of input
  let mut lines = input.lines();
  let instructions = lines.next().expect("No instructions found");
  let total_instructions: i32 = instructions.len() as i32;
  lines.next(); // Empty line

  let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();
  let mut lines_storage: Vec<String> = Vec::new();

  for l in lines {
    let mut line = l.replace(" ", "");
    line = line.replace("(", "");
    line = line.replace(")", "");
    lines_storage.push(line);
  }

  for line in &lines_storage {
    let parts: Vec<&str> = line.split("=").collect();
    let name = parts[0];
    let elements: Vec<&str> = parts[1].split(",").collect();
    nodes.insert(name, elements);
  }

  let mut steps: i32 = 0;
  let mut current = "AAA";
  while current != "ZZZ" {
    let instruction = instructions
      .chars()
      .nth(steps as usize % total_instructions as usize)
      .expect("No instruction found");
    if instruction == 'L' {
      current = nodes.get(current).expect("No node found")[0];
    } else {
      current = nodes.get(current).expect("No node found")[1];
    }
    steps += 1;
  }
  println!("{}", steps);
}
