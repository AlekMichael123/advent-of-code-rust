use std::collections::HashMap;

pub mod year2022;

pub fn get_year_solutions() -> HashMap<u16, HashMap<u8, fn(&str)>> {
  let year_solutions: [(u16, HashMap<u8, fn(&str)>); 1] = [
    (2022, year2022::get_solutions()),
  ];

  year_solutions.into_iter().collect()
}