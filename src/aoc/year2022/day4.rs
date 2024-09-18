use super::util::IntRange;

pub fn main(data: &str) {
  let ranges = parse_input(data);
  println!("Part 1 -- count number of pairs which one range completely overlaps -- {}", part1(&ranges));
  println!("Part 2 -- count number of pairs which one range partially overlaps -- {}", part2(&ranges));
}

fn part1(ranges: &Vec<(IntRange, IntRange)>) -> usize {
  ranges
    .iter()
    .filter(|(a, b)| a.completely_overlaps(b) || b.completely_overlaps(a))
    .count()
}

fn part2(ranges: &Vec<(IntRange, IntRange)>) -> usize {
  ranges
    .iter()
    .filter(|(a, b)| a.partially_overlaps(b) || b.partially_overlaps(a))
    .count()
}

fn parse_input(data: &str) -> Vec<(IntRange, IntRange)> {
  data
    .lines()
    .map(|line| {
      let mut line = line.split(",");
      (
        IntRange::try_from(line.next().expect("parse_input: input format error"))
          .expect("parse_input: a try_from failed"), 
        IntRange::try_from(line.next().expect("parse_input: input format error"))
          .expect("parse_input: b try_from failed"),
      )
    })
    .collect()
}