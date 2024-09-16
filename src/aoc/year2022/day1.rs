pub fn main(data: &str) {
  let mut calories = parse_data(data);

  println!("Part 1");
  
  let max = part1(&calories);
  println!("The elf holding the most is holding: {max}");
  
  println!("Part 2");
  
  let sum_of_highest_three = part2(&mut calories);
  println!("The sum of the highest three is: {sum_of_highest_three}"); 
}

fn part1(calories: &Vec<u32>) -> &u32 {
  calories.iter().max().unwrap()
}

fn part2(calories: &mut Vec<u32>) -> u32 {
  calories.sort();
  calories.reverse();
  let first_three = &calories[0..3];
  first_three.iter().sum()
}

fn parse_data(data: &str) -> Vec<u32> {
  data
    .split("\n\n")
    .map(|elf| elf.lines().map(|cal| cal.parse::<u32>().unwrap()).sum::<u32>())
    .collect()
}