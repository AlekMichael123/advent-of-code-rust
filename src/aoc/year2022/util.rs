use std::num::ParseIntError;

#[derive(Debug)]
pub struct IntRange {
  first: u8,
  last: u8,
}

impl IntRange {
  pub fn completely_overlaps(&self, range: &IntRange) -> bool {
    self.first <= range.first && self.last >= range.last
  }

  pub fn partially_overlaps(&self, range: &IntRange) -> bool {
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

#[derive(Debug)]
pub enum IntRangeError {
  InvalidParse(ParseIntError),
  InvalidInput,
}

impl From<ParseIntError> for IntRangeError {
  fn from(err: ParseIntError) -> Self {
    Self::InvalidParse(err)
  }
}
