use std::cmp::Ordering;

// vs: A for Rock, B for Paper, and C for Scissors.
// you: X for Rock, Y for Paper, and Z for Scissors
// The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) 
// plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

pub fn main(data: &str) {
  let plays = parse_input(data);
  println!("Part 1 -- Get final score for myself -- Score = {}", part1(&plays));
  println!("Part 2");
}

fn part1(plays: &Vec<(Play, Play)>) -> u16 {
  plays
    .iter()
    .map(|(opp, mine)| mine.win_value(opp) as u16)
		.sum()
}

fn parse_input(data: &str) -> Vec<(Play, Play)> {
  data
    .lines()
    .map(|line| {
      let plays: Vec<Play> = line.split(" ").map(|hand| Play::from(hand)).collect();
      match plays[..] {
        [opp, mine] => (opp, mine),
        _ => panic!("parse_input: Incorrect amount of hands given. This shouldn't have happened."),
      }
    })
    .collect()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
enum Play {
  Rock,
  Paper,
  Scissors,
}

impl Play {
	fn value(&self) -> u8 {
		match self {
			Play::Rock => 1,
			Play::Paper => 2,
			Play::Scissors => 3,
		}
	}

	fn win_value(&self, opp: &Play) -> u8 {
		self.value() + match self.cmp(opp) {
			Ordering::Less => 0, // I lose :C
			Ordering::Equal => 3, // Draw
			Ordering::Greater => 6, // I won!!! :DDD
		}
	}
}

impl Ord for Play {
  fn cmp(&self, other: &Self) -> Ordering {
		use Ordering::{Equal, Greater, Less};
		use Play::{Rock, Paper, Scissors};
    match (self, other) {
			(Rock, Rock) => Equal,
			(Rock, Paper) => Less,
			(Rock, Scissors) => Greater,
			(Paper, Rock) => Greater,
			(Paper, Paper) => Equal,
			(Paper, Scissors) => Less,
			(Scissors, Rock) => Less,
			(Scissors, Paper) => Greater,
			(Scissors, Scissors) => Equal,
		}
  }
}

impl From<&str> for Play {
  fn from(value: &str) -> Self {
    match value {
      "A" | "X" => Play::Rock,
      "B" | "Y" => Play::Paper,
      "C" | "Z" => Play::Scissors,
      _ => panic!("Play::from: Invalid play. This shouldn't have happened.")
    }
  }
}