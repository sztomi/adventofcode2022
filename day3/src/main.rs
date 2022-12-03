use std::collections::HashSet;

use common::{get_lines, Result};

fn priority(ch: char) -> u32 {
  if ch.is_uppercase() {
    (ch as u32) - ('A' as u32) + 27
  } else {
    (ch as u32) - ('a' as u32) + 1
  }
}

fn main() -> Result<()> {
  let lines = get_lines("day3/input.txt")?;
  let part1: u32 = lines
    .iter()
    .map(|line| {
      (
        HashSet::from_iter(line[0..line.len() / 2].chars()),
        HashSet::from_iter(line[line.len() / 2..].chars()),
      )
    })
    .flat_map(|bags: (HashSet<char>, HashSet<char>)| {
      let ints = bags.0.intersection(&bags.1).cloned();
      ints.collect::<Vec<char>>()
    })
    .map(priority)
    .sum();
  println!("Part1: {part1}");

  let part2: u32 = lines
    .chunks(3)
    .map(|group| {
      group
        .iter()
        .map(|s| s.to_string())
        .reduce(|acc, it| {
          let acc: HashSet<char> = HashSet::from_iter(acc.chars());
          let it: HashSet<char> = HashSet::from_iter(it.chars());
          acc.intersection(&it).cloned().collect()
        })
        .iter()
        .next()
        .unwrap()
        .clone()
    })
    .map(|s| priority(s.chars().next().unwrap()))
    .sum();
  println!("Part2: {part2}");

  Ok(())
}
