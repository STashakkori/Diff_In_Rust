use std::env;
use std::path::Path;
use diff::lines;

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

  let diff = lines(&file1, &file2);

  let mut left_lines = Vec::new();
  let mut right_lines = Vec::new();
  let mut changed = false;

  let mut left_line_num = 1;
  let mut right_line_num = 1;

  for change in &diff {
    match change {
      diff::Result::Left(l) => {
        if !changed {
          left_lines.push(format!(
            "{},{}c{},{}",
            left_line_num,
            left_line_num + l.len() - 1,
            right_line_num,
            right_line_num
           ));
          changed = true;
        }
      left_lines.push(format!(
        "\x1B[38;5;129m\x1B[48;5;228m> \x1B[1;35m{}\x1B[0m",
        l
      ));
      left_line_num += 1;
    }
      diff::Result::Both(_, _) => {
        left_line_num += 1;
        right_line_num += 1;
      }
      diff::Result::Right(r) => {
        if !changed {
          left_lines.push(format!(
            "{},{}c{},{}",
            left_line_num,
            left_line_num,
            right_line_num,
            right_line_num + r.len() - 1
           ));
          changed = true;
        }
        right_lines.push(format!(
          "\x1B[38;5;29m\x1B[48;5;228m< {}\x1B[0m",
          r
        ));
        right_line_num += 1;
      }
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
