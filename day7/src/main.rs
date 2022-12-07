use std::collections::{BinaryHeap, HashMap};

use common::{get_lines, Result};

const MAX_SIZE: i64 = 100000;
const TOTAL_DISK: i64 = 70000000;
const NEEDED_DISK: i64 = 30000000;

enum ShellLine {
  ChangeDir(String),
  GoUp,
  ListDir,
  Dir(String),
  File { size: i64, _name: String },
}

#[allow(clippy::ptr_arg)]
fn parse_line(line: &String) -> ShellLine {
  let line = line.trim();
  if line == "$ cd .." {
    ShellLine::GoUp
  } else if line == "$ ls" {
    ShellLine::ListDir
  } else if line.starts_with("$ cd") {
    ShellLine::ChangeDir(line.split(' ').last().unwrap().to_string())
  } else if line.starts_with("dir") {
    ShellLine::Dir(line.split(' ').last().unwrap().to_string())
  } else {
    let mut parts = line.split(' ');
    ShellLine::File {
      size: parts.next().unwrap().parse().unwrap(),
      _name: parts.next().unwrap().to_string(),
    }
  }
}

fn go_up(dir_stack: &mut Vec<String>, sizes: &mut HashMap<String, i64>) {
  let key = dir_stack.join("/");
  dir_stack.pop();
  let prev = *sizes.entry(key).or_default();
  let key = dir_stack.join("/");
  let curr = sizes.entry(key).or_default();
  *curr += prev;
}

fn main() -> Result<()> {
  let lines = get_lines("day7/input.txt")?;
  let parsed_lines: Vec<_> = lines.iter().map(parse_line).collect();

  let mut sizes = HashMap::new();
  let mut dir_stack = Vec::new();

  for line in parsed_lines {
    match line {
      ShellLine::ChangeDir(dir) => dir_stack.push(dir.to_string()),
      ShellLine::GoUp => go_up(&mut dir_stack, &mut sizes),
      ShellLine::File { size, _name } => {
        let key = dir_stack.join("/");
        let curr = sizes.entry(key).or_default();
        *curr += size;
      }
      _ => (),
    }
  }

  while dir_stack.len() != 1 {
    go_up(&mut dir_stack, &mut sizes);
  }

  let part1: i64 = sizes
    .iter()
    .filter(|(_, size)| **size <= MAX_SIZE)
    .map(|(_, size)| size)
    .sum();
  println!("Part1: {part1}");

  let available = TOTAL_DISK - sizes.get("/").unwrap();
  let needed = NEEDED_DISK - available;
  let sizes: BinaryHeap<i64> = sizes.iter().map(|(_, v)| *v).collect();
  let part2 = sizes.iter().rev().copied().find(|v| *v >= needed).unwrap();
  println!("Part2: {part2}");

  Ok(())
}
