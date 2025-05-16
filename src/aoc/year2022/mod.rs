use std::collections::HashMap;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;

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
    (8, day8::main),
    (9, day9::main),
    (10, day10::main),
    (11, day11::main),
    (12, day12::main),
    (13, day13::main),
    (14, day14::main),
    (15, day15::main),
    (16, day16::main),
    (17, day17::main),
  ];

  solutions.into_iter().collect()
}
