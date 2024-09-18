type InputType = Vec<(String, String)>;

pub fn main(data: &str) {
  let compartments = parse_input(data);
  println!("Part 1 -- Find sum of matching items in rucksacks -- Sum = {}", part1(&compartments))
}

fn part1(compartments: &InputType) -> u16 {
  compartments
    .iter()
    .map(|(comp_a, comp_b)| {
      let res = comp_b
        .chars()
        .find(|letter| comp_a.contains(&letter.to_string()))
        .expect("Cannot find matching pair");
      if res.is_lowercase() {
        res as u16 - 96
      } else {
        res as u16 - 38
      }
    })
    .sum()
}

fn parse_input(data: &str) -> InputType {
  data
    .lines()
    .map(|line| {
      let n = line.len();
      let half = n / 2;
      (line[0..half].to_string(), line[half..].to_string())
    })
    .collect()
}