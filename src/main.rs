use std::fs::read_to_string;
use std::env;

use advent_of_code_rust::use_day;

fn main() {
  let Ok((year, day)) = get_args() else {
    panic!("Error: get_args failed!")
  };
  match read_input_file() {
    Ok(data) => match use_day(year, day, &data) {
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

fn get_args() -> Result<(u16, u8), ()> {
  let args: Vec<String> = env::args().collect();
  let mut args = args.iter();
  let (Some(_), Some(year), Some(day), None) = (args.next(), args.next(), args.next(), args.next()) else {
    println!("Not enough arguments passed.");
    return Err(());
  };
  let (Ok(year), Ok(day)) = ((*year).parse::<u16>(), (*day).parse::<u8>()) else {
    println!("Parsing error occured");
    return Err(());
  };
  Ok((year, day))
}