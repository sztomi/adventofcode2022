use common::{get_chars, Result};
use std::collections::HashSet;

fn all_diff(chars: &[char]) -> bool {
  let cset: HashSet<&char> = chars.iter().collect();
  cset.len() == chars.len()
}

fn main() -> Result<()> {
  let chars: Vec<_> = get_chars("day6/input.txt")?;
  let chars: &[_] = &chars;
  let ans = |cnt| chars.windows(cnt).take_while(|w| !all_diff(w)).count() + cnt;

  let part1 = ans(4);
  println!("Part1: {part1}");
  let part2 = ans(14);
  println!("Part2: {part2}");

  Ok(())
}
