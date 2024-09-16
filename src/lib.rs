use std::time::Instant;

pub mod aoc;

pub fn use_day(year: u16, day: u8, data: &str) -> Result<(), &str> {
  let solutions = aoc::get_year_solutions();
  let Some(solutions) = solutions.get(&year) else { 
    return Err("Year does not exist or is not implemented yet.");
  };
  match solutions.get(&day) {
    Some(solution) => {
      let start = Instant::now();
      solution(data);
      println!("Total time elapsed: {:.2?}", start.elapsed());
      Ok(())
    },
    None => Err("Day does not exist or is not implemented yet."),
  }
}