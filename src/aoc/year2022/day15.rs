use std::collections::HashSet;

pub fn main(data: &str) {
  let sensors = parse_data(data);
  // println!("Sensors = {:?}", sensors[6]);

  println!("Part 1");
  // let y = 10;
  let y = 2_000_000;
  let count = part1(&sensors, y);
  println!("Count = {count}");

  println!("Part 2");
  // let max = 20;
  let max = 4_000_000;
  let tuning_frequency = part2(&sensors, max);
  println!("Tuning frequency = {tuning_frequency}");
}

fn part1(sensors: &Vec<Sensor>, y: i64) -> i64 {
  // gather all ranges that are covered by sensors on y
  let ranges = Sensor::merge_x_covered_ranges_for_y(y, &mut sensors.clone());

  // find all beacons that are on y to remove them from the count
  let beacons_on_y = sensors
    .iter()
    .filter(|sensor| sensor.beacon.y == y)
    .map(|sensor| sensor.beacon.x)
    .collect::<HashSet<i64>>()
    .iter()
    .count();

  // count the number of positions that are covered by sensors on y
  let position_count = ranges
    .iter()
    .map(|range| range.y - range.x + 1)
    .sum::<i64>();
  
  position_count - beacons_on_y as i64
}

// this is a brute force crap solution but i don't care
fn part2(sensors: &Vec<Sensor>, max: i64) -> i64 {
  for y in 0..=max {
    // gather all ranges that are covered by sensors on y
    let ranges = Sensor::merge_x_covered_ranges_for_y(y, &mut sensors.clone());
    let mut x = 0;
    // find the first range that is not covered by any sensor
    for range in ranges {
      if range.x > x {
        return x * 4_000_000 + y;
      } else {
        x = range.y + 1;
      }
    }
  }
  unreachable!("No solution found")
}

fn parse_data(data: &str) -> Vec<Sensor> {
  let data = data.replace([',', ':'], "");
  let mut sensors = Vec::new();
  for line in data.lines() {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let pos_x = parts[2].split('=').nth(1).unwrap().parse::<i64>().unwrap();
    let pos_y = parts[3].split('=').nth(1).unwrap().parse::<i64>().unwrap();
    let beacon_x = parts[8].split('=').nth(1).unwrap().parse::<i64>().unwrap();
    let beacon_y = parts[9].split('=').nth(1).unwrap().parse::<i64>().unwrap();

    sensors.push(Sensor::new(
      Range::new(pos_x, pos_y),
      Range::new(beacon_x, beacon_y),
    ));
  }
  sensors
}

#[derive(Debug, Clone, Copy)]
struct Range {
  x: i64,
  y: i64,
}

impl Range {
  fn new(x: i64, y: i64) -> Self {
    Self { x, y }
  }
}

#[derive(Debug, Clone)]
struct Sensor {
  pos: Range,
  beacon: Range,
}

impl Sensor {
  fn new(pos: Range, beacon: Range) -> Self {
    Self { pos, beacon }
  }

  fn total_range(&self) -> i64 {
    (self.pos.x - self.beacon.x).abs() + (self.pos.y - self.beacon.y).abs()
  }

  fn x_covered_range_for_y(&self, y: i64) -> Option<Range> {
    let total_range = self.total_range();
    let distance_to_y = (self.pos.y - y).abs();

    if distance_to_y <= total_range {
      let x_range = total_range - distance_to_y;
      Some(Range::new(self.pos.x - x_range, self.pos.x + x_range))
    } else {
      None
    }
  }

  // shoutout to for this idea, https://github.com/WinterCore/aoc2022/blob/main/day15/main.rs#L82
  // at first i was just using a HashSet to store the ranges, but that was slow and sucked butt
  fn merge_x_covered_ranges_for_y(y: i64, ranges: &mut Vec<Sensor>) -> Vec<Range> {
    let mut ranges = ranges
      .iter()
      .filter_map(|sensor| sensor.x_covered_range_for_y(y))
      .collect::<Vec<Range>>();

    ranges.sort_by_key(|r| r.x);

    let mut result = Vec::new();

    let mut acc = ranges[0];
    for i in 1..ranges.len() {
      let range = ranges[i];
      if acc.y >= (range.x-1) {
        acc.y = acc.y.max(range.y);
      } else {
        result.push(acc);
        acc = range;
      }
    }

    result.push(acc);

    result
  }
}