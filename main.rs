use std::env;
use std::path::Path;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 3 {
    eprintln!("Usage: {} <file1> <file2>", &args[0]);
    std::process::exit(1);
  }

  let file1_path = &args[1];
  let file2_path = &args[2];

  let file1 = std::fs::read_to_string(Path::new(file1_path)).unwrap();
  let file2 = std::fs::read_to_string(Path::new(file2_path)).unwrap();

  let lines1: Vec<&str> = file1.split('\n').collect();
  let lines2: Vec<&str> = file2.split('\n').collect();

  let mut line_num1 = 1;
  let mut line_num2 = 1;
  let mut changed = false;
  let mut left_lines = Vec::new();
  let mut right_lines = Vec::new();

  while line_num1 <= lines1.len() || line_num2 <= lines2.len() {
    let line1 = if line_num1 <= lines1.len() { lines1[line_num1 - 1] }
                else { "" };

    let line2 = if line_num2 <= lines2.len() { lines2[line_num2 - 1] }
                else { "" };

    if line1 == line2 {
      left_lines.push(String::from(line1));
      right_lines.push(String::from(line1));
      line_num1 += 1;
      line_num2 += 1;
    }
    else { // Lines differ
      if !changed {
        changed = true;
        left_lines.push(format!(
          "{},{}c{},{}",
          line_num1,
          lines1.len(),
          line_num2,
          lines2.len()
        ));
      }
    if !line1.is_empty() {
      left_lines.push(format!(
        "\x1B[38;5;129m\x1B[48;5;228m> \x1B[1;35m{}\x1B[0m",
        line1
      ));
    }
    if !line2.is_empty() {
      right_lines.push(format!(
        "\x1B[38;5;29m\x1B[48;5;228m< {}\x1B[0m",
        line2
      ));
    }
    line_num1 += 1;
    line_num2 += 1;
    }
  }

  for (i, line) in left_lines.iter().enumerate() {
    if !line.is_empty() {
      if i == left_lines.len() - 1 { print!("{}", line); }
      else { println!("{}", line); }
    }
  }

  if changed { println!("---") }

  for (i, line) in right_lines.iter().enumerate() {
    if !line.is_empty() {
      if i == right_lines.len() - 1 { print!("{}", line); }
      else { println!("{}", line); }
    }
  }
}
