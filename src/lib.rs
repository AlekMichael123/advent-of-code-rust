pub mod aoc;

pub fn use_day(year: u16, day: u8, data: &str) -> Result<(), &str> {
  let solutions = aoc::get_year_solutions();
  let Some(solutions) = solutions.get(&year) else { 
    return Err("Year does not exist or is not implemented yet.");
   };
  match solutions.get(&day) {
    Some(solution) => Ok(solution(data)),
    None => Err("Day does not exist or is not implemented yet."),
  }
}