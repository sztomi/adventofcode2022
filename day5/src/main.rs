use std::collections::VecDeque;

use common::{get_lines, Result};

fn main() -> Result<()> {
  let lines = get_lines("day5/input.txt")?;

  let boxes: Vec<_> = lines.iter().take_while(|line| line.contains('[')).collect();
  let boxes: Vec<_> = boxes.iter().rev().collect();

  let mut stacks1 = Vec::new();

  for col in 0..9 {
    stacks1.push(VecDeque::new());
    for row in 0..boxes.len() {
      let ch = boxes[row].chars().nth(1 + col * 4).unwrap();
      if ch != ' ' {
        stacks1[col].push_back(ch);
      }
    }
  }

  let prg: Vec<_> = lines
    .iter()
    .skip_while(|line| !line.starts_with("move"))
    .map(|line| {
      line
        .split(' ')
        .skip(1)
        .step_by(2)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
    })
    .map(|v| (v[0], v[1], v[2]))
    .collect();

  let mut stacks2 = stacks1.clone();

  for (cnt, from, to) in prg.iter() {
    for _ in 0..*cnt {
      let val = stacks1[*from - 1].pop_back().unwrap();
      stacks1[*to - 1].push_back(val);
    }
  }

  print!("Part1: ");
  for st in &stacks1 {
    print!("{}", st.back().unwrap_or(&'_'));
  }
  println!();

  for (cnt, from, to) in prg.iter() {
    let chunk: Vec<_> = stacks2[*from - 1].iter().rev().take(*cnt).copied().collect();
    for it in chunk.iter().rev() {
      stacks2[*to - 1].push_back(*it);
    }
    let len = stacks2[*from - 1].len();
    stacks2[*from - 1].resize(len - *cnt, 0 as char);
  }

  print!("Part2: ");
  for st in &stacks2 {
    print!("{}", st.back().unwrap_or(&'_'));
  }
  println!();

  Ok(())
}
