use std::cmp::Ordering;

pub fn main(data: &str) {
  let plays = parse_input(data);
  println!("Part 1 -- Get final score for myself -- Score = {}", part1(&plays));
  println!("Part 2 -- Adjust my strategy then get final score -- Score = {}", part2(&plays));
}

/// Year 2022 - Day 2 - Part 1\
/// Need to find the final score if I play to win each game.
fn part1(plays: &Vec<(Play, Play)>) -> u16 {
  plays
    .iter()
    .map(|(opp, mine)| mine.game_score(opp) as u16)
    .sum()
}

/// Year 2022 - Day 2 - Part 2\
/// Need to find the final score for me if I follow the elves' strategies 
fn part2(plays: &Vec<(Play, Play)>) -> u16 {
  plays
    .iter()
    .map(|(opp, mine)| mine.win_value_with_strategy(opp) as u16)
    .sum()
}

/// Groups input to each game's self & opponent's hand
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

/// Used to denote what strategy to use based off the input.\
/// ie, If the input said to play Rock, we need to lose. 
enum Strategy {
  Lose,
  Draw,
  Win,
}

impl From<&Play> for Strategy {
  fn from(play: &Play) -> Self {
    match play {
      Play::Rock => Strategy::Lose,
      Play::Paper => Strategy::Draw,
      Play::Scissors => Strategy::Win,
    }
  }
}

/// Represents the choice between rock, paper, or scissor.\ 
/// Contains methods used to get a game's final score.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd)]
enum Play {
  Rock,
  Paper,
  Scissors,
}

impl Play {
  /// Get numeric value of a play
	fn value(&self) -> u8 {
		match self {
			Play::Rock => 1,
			Play::Paper => 2,
			Play::Scissors => 3,
		}
	}

  /// Computes a game's final score for self
	fn game_score(&self, opp: &Play) -> u8 {
		self.value() + match self.cmp(opp) {
			Ordering::Less => 0, // I lose :C
			Ordering::Equal => 3, // Draw
			Ordering::Greater => 6, // I won!!! :DDD
		}
	}

  /// Computes final score with the intended strategy in mind. 
  fn win_value_with_strategy(&self, opp: &Play) -> u8 {
    let play = match Strategy::from(self) {
      Strategy::Lose => Play::to_lose(opp),
      Strategy::Draw => opp,
      Strategy::Win => Play::to_win(opp),
    };
    play.game_score(opp)
  }

  /// Determines what will lose to play
  fn to_lose(play: &Play) -> &Play {
    match play {
      Play::Rock => &Play::Scissors,
      Play::Paper => &Play::Rock,
      Play::Scissors => &Play::Paper,
    }
  }

  /// Determines what will win with play
  fn to_win(play: &Play) -> &Play {
    match play {
      Play::Rock => &Play::Paper,
      Play::Paper => &Play::Scissors,
      Play::Scissors => &Play::Rock,
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