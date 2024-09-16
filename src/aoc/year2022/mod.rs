use std::collections::HashMap;

pub mod day1;

pub fn get_solutions() -> HashMap<u8, fn(&str)> {
  let solutions: Vec<(u8, fn(&str))> = vec![
    (1, day1::main),
  ];

  solutions.into_iter().collect()
}