use common::{get_lines, Result};

#[derive(Debug)]
struct Range(u32, u32);

fn str2range(s: &str) -> Range {
  let mut parts = s.split('-');
  let a: u32 = parts.next().unwrap().parse().unwrap();
  let b: u32 = parts.next().unwrap().parse().unwrap();
  Range(a, b)
}

fn contains(lhs: &Range, rhs: &Range) -> bool {
  (lhs.0 >= rhs.0 && lhs.1 <= rhs.1) || (rhs.0 >= lhs.0 && rhs.1 <= lhs.1)
}

fn overlaps(lhs: &Range, rhs: &Range) -> bool {
  (rhs.0..=rhs.1).contains(&lhs.0)
    || (rhs.0..=rhs.1).contains(&lhs.1)
    || (lhs.0..=lhs.1).contains(&rhs.0)
    || (lhs.0..=lhs.1).contains(&rhs.1)
}

fn main() -> Result<()> {
  let lines = get_lines("day4/input.txt")?;

  let part1 = lines
    .iter()
    .map(|line| line.split(',').collect::<Vec<&str>>())
    .map(|it| (str2range(it[0]), str2range(it[1])))
    .filter(|it| contains(&it.0, &it.1))
    .count();
  println!("Part1: {part1}");

  let part2 = lines
    .iter()
    .map(|line| line.split(',').collect::<Vec<&str>>())
    .map(|it| (str2range(it[0]), str2range(it[1])))
    .filter(|it| overlaps(&it.0, &it.1))
    .count();
  println!("Part2: {part2}");

  Ok(())
}
