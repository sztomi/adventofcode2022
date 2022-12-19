use core::panic;
use std::collections::HashSet;

use common::{get_lines, Dir, Point, Result};
use itertools::Itertools;

fn calc_tail(t: &Point, h: &Point) -> Point {
  let diff = h - t;
  let (dx, dy) = match diff.coords() {
    // covered | touching
    (0, 0) | (0, 1) | (1, 0) | (0, -1) | (-1, 0) | (-1, -1) | (1, 1) | (1, -1) | (-1, 1) => (0, 0),
    // same col/row
    (0, 2) => (0, 1),
    (2, 0) => (1, 0),
    (0, -2) => (0, -1),
    (-2, 0) => (-1, 0),
    // diagonal
    (2, 1) | (1, 2) | (2, 2) => (1, 1),
    (2, -1) | (1, -2) | (2, -2) => (1, -1),
    (-2, 1) | (-1, 2) | (-2, 2) => (-1, 1),
    (-2, -1) | (-1, -2) | (-2, -2) => (-1, -1),
    _ => panic!("at the disco"),
  };
  t + Dir(dx, dy)
}

fn main() -> Result<()> {
  let lines = get_lines("day9/input.txt")?;
  let prg = lines
    .iter()
    .map(|ln| {
      let parts = ln.trim().split(' ').collect_vec();
      let dir = match parts[0] {
        "U" => Dir(0, 1),
        "R" => Dir(1, 0),
        "D" => Dir(0, -1),
        "L" => Dir(-1, 0),
        _ => panic!("at the disco"),
      };
      (dir, parts[1].parse::<i64>().unwrap())
    })
    .collect_vec();

  let mut prev_head = Point(0, 0);
  let mut prev_tail = Point(0, 0);
  let points: HashSet<Point> = prg
    .iter()
    .flat_map(|(d, dist)| {
      (0..*dist)
        .into_iter()
        .map(|_| {
          prev_head += d;
          prev_tail = calc_tail(&prev_tail, &prev_head);
          prev_tail
        })
        .collect_vec()
    })
    .collect();
  let part1 = points.len();
  println!("Part1: {part1}");

  let mut knots = [Point(0, 0); 10];
  let mut points = HashSet::new();
  
  for (direction, dist) in prg.iter() {
    for _ in 0..*dist {
      knots[0] += direction;
      for j in 1..10 {
        knots[j] = calc_tail(&knots[j], &knots[j-1]);
      }
      points.insert(knots[9]);
    }
  }
  let part2 = points.len();
  println!("Part2: {part2}");

  Ok(())
}
