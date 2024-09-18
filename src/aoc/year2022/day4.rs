use std::num::ParseIntError;

pub fn main(data: &str) {
  let ranges = parse_input(data);
  println!("Part 1 -- count number of pairs which one range completely overlaps -- {}", part1(&ranges));
  println!("Part 2 -- count number of pairs which one range partially overlaps -- {}", part2(&ranges));
}

fn part1(ranges: &Vec<(IntRange, IntRange)>) -> usize {
  ranges
    .iter()
    .filter(|(a, b)| a.completely_overlaps(b) || b.completely_overlaps(a))
    .count()
}

fn part2(ranges: &Vec<(IntRange, IntRange)>) -> usize {
  ranges
    .iter()
    .filter(|(a, b)| a.partially_overlaps(b) || b.partially_overlaps(a))
    .count()
}

fn parse_input(data: &str) -> Vec<(IntRange, IntRange)> {
  data
    .lines()
    .map(|line| {
      let mut line = line.split(",");
      (
        IntRange::try_from(line.next().expect("parse_input: input format error"))
          .expect("parse_input: a try_from failed"), 
        IntRange::try_from(line.next().expect("parse_input: input format error"))
          .expect("parse_input: b try_from failed"),
      )
    })
    .collect()
}

#[derive(Debug)]
enum IntRangeError {
  InvalidParse(ParseIntError),
  InvalidInput,
}

impl From<ParseIntError> for IntRangeError {
  fn from(err: ParseIntError) -> Self {
    Self::InvalidParse(err)
  }
}

#[derive(Debug)]
struct IntRange {
  first: u8,
  last: u8,
}

impl IntRange {
  fn completely_overlaps(&self, range: &IntRange) -> bool {
    self.first <= range.first && self.last >= range.last
  }

  fn partially_overlaps(&self, range: &IntRange) -> bool {
    self.first <= range.first && self.last >= range.first 
    || self.first <= range.last && self.last >= range.last
  }
}

impl TryFrom<&str> for IntRange {
  type Error = IntRangeError;
  fn try_from(range: &str) -> Result<Self, Self::Error> {
    let mut range_iter = range.split("-").into_iter();
    if 
      let (Some(first), Some(last), None) = 
        (range_iter.next(), range_iter.next(), range_iter.next())
    {
      Ok(Self {
        first: first.parse()?,
        last: last.parse()?,
      })
    } else {
      Err(Self::Error::InvalidInput)
    }
  }
}