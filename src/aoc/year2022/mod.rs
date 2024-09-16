use std::collections::HashMap;

pub mod day1;

pub fn get_solutions() -> HashMap<u8, fn(&str)> {
  let solutions: [(u8, fn(&str)); 1] = [
    (1, day1::main),
  ];

  // Use the `into_iter` method to consume the array and collect directly
  solutions.into_iter().collect()
}