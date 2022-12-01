use common::{get_lines, Result};
use std::collections::BinaryHeap;

fn main() -> Result<()> {
  let lines = get_lines("day1/input.txt")?;

  let calories: BinaryHeap<u32> = lines
    .split(|line| line.is_empty())
    .map(|sublist| {
      sublist
        .iter()
        .map(|it| it.parse::<u32>().unwrap())
        .sum::<u32>()
    })
    .collect();

  let max_cal = calories.peek().unwrap();
  println!("Part1: {max_cal}");

  let top3_sum: u32 = calories.iter().take(3).sum();
  println!("Part2: {top3_sum}");

  Ok(())
}
