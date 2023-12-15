use std::collections::HashMap;
use std::collections::HashSet as Set;
use std::io::Read;

fn gcd(mut a: i64, mut b: i64) -> i64 {
  while b != 0 {
    let t = b;
    b = a % b;
    a = t;
  }
  return a;
}

fn main() {
  // Read strings from stdin until EOF
  let mut input = String::new();
  std::io::stdin().read_to_string(&mut input).unwrap();

  // Read first line of input
  let mut lines = input.lines();
  let instructions = lines.next().expect("No instructions found");
  let total_instructions: i32 = instructions.len() as i32;
  lines.next(); // Empty line

  let mut nodes: HashMap<String, Vec<String>> = HashMap::new();
  let mut final_nodes: Vec<String> = Vec::new();
  let mut start_nodes: Vec<String> = Vec::new();

  for line in lines {
    let line = line.replace(" ", "").replace("(", "").replace(")", "");
    let parts: Vec<&str> = line.split("=").collect();
    let name = parts[0].to_string();
    let elements: Vec<String> = parts[1].split(",").map(|s| s.to_string()).collect();
    nodes.insert(name.clone(), elements);
    match name.chars().last().expect("No last char") {
      'Z' => final_nodes.push(name.clone()),
      'A' => start_nodes.push(name),
      _ => {}
    }
  }

  let mut ending_steps: Set<i64> = Set::new();
  for start_node in &start_nodes {
    // println!("Start node: {}", start_node);
    let mut steps: i32 = 0;
    let mut current = start_node;
    let mut pos_instruction = 0;
    let mut current_instruction = instructions.chars().nth(0).expect("No instruction found");
    let mut states: Set<String> = Set::new();
    let mut state: String =
      current.to_string() + &current_instruction.to_string() + &pos_instruction.to_string();
    loop {
      if states.contains(&state) {
        // println!("Loop {} detected", state);
        break;
      }
      states.insert(state.clone());
      match current_instruction {
        'L' => current = &nodes.get(current).expect("No node found")[0],
        'R' => current = &nodes.get(current).expect("No node found")[1],
        _ => {}
      }
      steps += 1;
      if final_nodes.contains(&current) {
        // println!("Final node {} reached in {} steps", current, steps);
        ending_steps.insert(steps as i64);
      }
      pos_instruction = steps % total_instructions;
      current_instruction = instructions
        .chars()
        .nth(pos_instruction as usize)
        .expect("No instruction found");
      state = current.to_string() + &current_instruction.to_string() + &pos_instruction.to_string();
    }
  }

  // Find the LCM of all the ending steps
  let mut lcm: i64 = 1;
  for ending_step in &ending_steps {
    lcm = lcm * ending_step / gcd(lcm, *ending_step);
  }
  println!("LCM: {}", lcm);
}
