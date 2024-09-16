pub fn main(data: &str) {
  println!("Part 1");
  let calories = parse_data(data);
  let max = calories.iter().max().expect("Max error");
  println!("The elf holding the most is holding: {max}");
  println!("Part 2");
}

fn part1() {}

fn parse_data(data: &str) -> Vec<u32> {
  let mut acc: u32 = 0;
  let mut calories: Vec<u32> = vec![];

  data
    .lines()
    .for_each(|line| {
      if line == "" {
        calories.push(acc);
        acc = 0;
      } else {
        acc += line.parse::<u32>().expect("Parsing error");
      }
    });

  calories
}