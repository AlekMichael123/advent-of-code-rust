use std::num::ParseIntError;

pub fn main(data: &str) {

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

struct IntRange {
  first: u8,
  last: u8,
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