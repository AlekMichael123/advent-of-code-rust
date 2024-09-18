use std::collections::HashMap;

pub mod day1;
pub mod day2;
pub mod day3;

pub fn get_solutions() -> HashMap<u8, fn(&str)> {
  let solutions: Vec<(u8, fn(&str))> = vec![
    (1, day1::main),
    (2, day2::main),
    (3, day3::main),
  ];

  solutions.into_iter().collect()
}