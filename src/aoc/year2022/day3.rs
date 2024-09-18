pub fn main(data: &str) {
  let compartments = parse_input(data);
  println!("Part 1 -- Find sum of matching items in rucksacks -- Sum = {}", part1(&compartments));
  println!("Part 2 -- Find sum of matching items in groups of three -- Sum = {}", part2(&compartments));
}

fn part1(compartments: &Vec<String>) -> u16 {
  compartments
    .iter()
    .map(|line| {
      let n = line.len();
      let half = n / 2;
      (line[0..half].to_string(), line[half..].to_string())
    })
    .map(|(comp_a, comp_b)| {
      let res = comp_b
        .chars()
        .find(|letter| comp_a.contains(&letter.to_string()))
        .expect("Cannot find matching pair.");
      if res.is_lowercase() {
        res as u16 - 96
      } else {
        res as u16 - 38
      }
    })
    .sum()
}

fn part2(compartments: &Vec<String>) -> u16 {
  let mut comp_iter = compartments.iter();
  let mut res: u16 = 0;

  while 
    let (Some(comp_a), Some(comp_b), Some(comp_c)) = 
        (comp_iter.next(), comp_iter.next(), comp_iter.next()) 
  {
    let all_matching_letter = 
      comp_a
        .chars()
        .find(|letter| {
          let letter = letter.to_string();
          comp_b.contains(&letter) && comp_c.contains(&letter)
        })
        .expect("Cannot find matching letter.");
    res += 
      if all_matching_letter.is_lowercase() {
        all_matching_letter as u16 - 96
      } else {
        all_matching_letter as u16 - 38
      }
  }
  res
}

fn parse_input(data: &str) -> Vec<String> {
  data
    .lines()
    .map(|line| line.to_string())
    .collect()
}