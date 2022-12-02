use common::{get_lines, Result};

const WIN: u32 = 6;
const DRAW: u32 = 3;

#[derive(Clone, Copy)]
#[repr(u32)]
enum Move {
  Rock = 1,
  Paper = 2,
  Scissors = 3,
}

#[derive(Clone, Copy)]
enum Outcome {
  Win,
  Lose,
  Draw,
}

fn ch2move(ch: &str, base: char) -> Move {
  let val = (ch.chars().next().unwrap() as u32) - (base as u32) + 1;
  match val {
    1 => Move::Rock,
    2 => Move::Paper,
    3 => Move::Scissors,
    _ => panic!("oh no"),
  }
}

fn ch2outcome(ch: &str) -> Outcome {
  let ch = ch.chars().next().unwrap();
  match ch {
    'X' => Outcome::Lose,
    'Y' => Outcome::Draw,
    'Z' => Outcome::Win,
    _ => panic!("sorry"),
  }
}

fn eval(opp: Move, me: Move) -> u32 {
  match me {
    Move::Rock => match opp {
      Move::Rock => (me as u32) + DRAW,
      Move::Paper => me as u32,
      Move::Scissors => (me as u32) + WIN,
    },
    Move::Paper => match opp {
      Move::Rock => (me as u32) + WIN,
      Move::Paper => (me as u32) + DRAW,
      Move::Scissors => me as u32,
    },
    Move::Scissors => match opp {
      Move::Rock => me as u32,
      Move::Paper => (me as u32) + WIN,
      Move::Scissors => (me as u32) + DRAW,
    },
  }
}

fn get_move(opp: Move, outcome: Outcome) -> Move {
  match outcome {
    Outcome::Win => match opp {
      Move::Rock => Move::Paper,
      Move::Paper => Move::Scissors,
      Move::Scissors => Move::Rock,
    },
    Outcome::Lose => match opp {
      Move::Rock => Move::Scissors,
      Move::Paper => Move::Rock,
      Move::Scissors => Move::Paper,
    },
    Outcome::Draw => opp,
  }
}

fn main() -> Result<()> {
  let lines = get_lines("day2/input.txt")?;
  let game: Vec<(Move, Move)> = lines
    .iter()
    .map(|line| {
      let mut moves = line.trim().split(' ');
      let opponent = ch2move(moves.next().unwrap(), 'A');
      let me = ch2move(moves.next().unwrap(), 'X');
      (opponent, me)
    })
    .collect();

  let score: u32 = game.iter().map(|r| eval(r.0, r.1)).sum();
  println!("Part1: {score}");

  let game: Vec<(Move, Move)> = lines
    .iter()
    .map(|line| {
      let mut moves = line.trim().split(' ');
      let opponent = ch2move(moves.next().unwrap(), 'A');
      let me = get_move(opponent, ch2outcome(moves.next().unwrap()));
      (opponent, me)
    })
    .collect();

  let score: u32 = game.iter().map(|r| eval(r.0, r.1)).sum();
  println!("Part2: {score}");

  Ok(())
}
