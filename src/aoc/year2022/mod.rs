use std::collections::HashMap;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub mod util;

pub fn get_solutions() -> HashMap<u8, fn(&str)> {
  let solutions: Vec<(u8, fn(&str))> = vec![
    (1, day1::main),
    (2, day2::main),
    (3, day3::main),
    (4, day4::main),
    (5, day5::main),
    (6, day6::main),
    (7, day7::main),
  ];

  solutions.into_iter().collect()
}
