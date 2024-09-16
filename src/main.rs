use std::fs::read_to_string;

use advent_of_code_rust::use_day;

fn main() {
  match read_input_file() {
    Ok(data) => match use_day(2022, 1, &data) {
      Ok(_) => (),
      Err(err) => println!("ERROR: {err}"),
    },
    Err(err) => println!("ERROR: {err}"),
  }  
}

fn read_input_file() -> Result<String, String> {
  match read_to_string("input.txt") {
    Ok(data) => Ok(data),
    Err(_) => Err("Could not find input file.".to_string()),
  }
}