use std::collections::HashSet;
use std::ops;

use common::{get_lines, Result};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Point(i64, i64);
#[derive(Debug, Clone, Copy)]
struct Dir(i64, i64);

struct Line<'a> {
  map: &'a Vec<Vec<i64>>,
  dir: Dir,
  point: Point,
}

fn oob(p: &Point, map: &Vec<Vec<i64>>) -> bool {
  p.0 < 0 || p.1 < 0 || p.0 >= map.len() as i64 || p.1 >= map[0].len() as i64
}

fn edge(p: &Point, map: &Vec<Vec<i64>>) -> bool {
  p.0 == 0 || p.1 == 0 || p.0 == (map.len() as i64) - 1 || p.1 == (map[0].len() as i64) - 1
}

impl Iterator for Line<'_> {
  type Item = i64;

  fn next(&mut self) -> Option<Self::Item> {
    if oob(&self.point, self.map) {
      return None;
    }
    let res = self.map[self.point.0 as usize][self.point.1 as usize];
    self.point += self.dir;
    Some(res)
  }
}

impl ops::AddAssign<Dir> for Point {
  fn add_assign(&mut self, rhs: Dir) {
    self.0 += rhs.0;
    self.1 += rhs.1;
  }
}

impl ops::Add<Dir> for Point {
  type Output = Point;

  fn add(self, rhs: Dir) -> Self::Output {
    Point(self.0 + rhs.0, self.1 + rhs.1)
  }
}

fn has_los(map: &Vec<Vec<i64>>, point: Point, dir: Dir) -> bool {
  let mut line = Line { map, dir, point };
  let mut first: Option<i64> = None;
  line.all(|x| {
    if let Some(first) = first {
      x < first
    } else {
      first = Some(x);
      true
    }
  })
}

fn score(map: &Vec<Vec<i64>>, point: Point, dir: Dir) -> usize {
  if edge(&point, map) || oob(&(point + dir), map) {
    return 0;
  }
  let first = map[point.0 as usize][point.1 as usize];
  let line = Line { map, dir, point };
  let mut stop = false;
  line
    .skip(1)
    .take_while(|x| {
      if !stop && x >= &first {
        stop = true;
        return true;
      }
      !stop
    })
    .count()
}

fn main() -> Result<()> {
  let lines = get_lines("day8/input.txt")?;
  let map: Vec<Vec<i64>> = lines
    .iter()
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect()
    })
    .collect();

  let directions = [Dir(-1, 0), Dir(1, 0), Dir(0, -1), Dir(0, 1)];
  let ylen = map.len();
  let xlen = map[0].len();
  let vis: HashSet<(usize, usize)> = (0..xlen)
    .cartesian_product(0..ylen)
    .filter(|(x, y)| {
      directions
        .iter()
        .any(|d| has_los(&map, Point(*x as i64, *y as i64), *d))
    })
    .collect();
  let part1 = vis.len();
  println!("Part1: {part1}");

  let part2 = (0..xlen)
    .cartesian_product(0..ylen)
    .map(|(x, y)| {
      directions
        .iter()
        .map(|d| score(&map, Point(x as i64, y as i64), *d))
        .collect_vec()
    })
    .map(|f| f.iter().product::<usize>())
    .max()
    .unwrap_or(0);
  println!("Part2: {part2}");

  Ok(())
}
