use std::{collections::HashSet, hash::Hash, vec};

pub fn main(data: &str) {
  let sensors = parse_data(data);
  // println!("Sensors = {:?}", sensors[6]);

  println!("Part 1");
  // let y = 10;
  let y = 2_000_000;
  let count = part1(&sensors, y);
  println!("Count = {count}");
}

fn part1(sensors: &Vec<Sensor>, y: i64) -> i64 {

  let ranges = Sensor::merge_x_covered_ranges_for_y(y, &mut sensors.clone());

  let beacons_on_y = sensors
    .iter()
    .filter(|sensor| sensor.beacon.y == y)
    .map(|sensor| sensor.beacon.x)
    .collect::<HashSet<i64>>()
    .iter()
    .count();

  let position_count = ranges
    .iter()
    .map(|range| range.1 - range.0 + 1)
    .sum::<i64>();
  
  position_count - beacons_on_y as i64
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
      Position::new(pos_x, pos_y),
      Position::new(beacon_x, beacon_y),
    ));
  }
  sensors
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Position {
  x: i64,
  y: i64,
}

impl Position {
  fn new(x: i64, y: i64) -> Self {
    Self { x, y }
  }
}

#[derive(Debug, Clone)]
struct Sensor {
  pos: Position,
  beacon: Position,
}

impl Sensor {
  fn new(pos: Position, beacon: Position) -> Self {
    Self { pos, beacon }
  }

  fn total_range(&self) -> i64 {
    (self.pos.x - self.beacon.x).abs() + (self.pos.y - self.beacon.y).abs()
  }

  fn x_covered_range_for_y(&self, y: i64) -> Option<(i64, i64)> {
    let total_range = self.total_range();
    let distance_to_y = (self.pos.y - y).abs();

    if distance_to_y <= total_range {
      let x_range = total_range - distance_to_y;
      Some((self.pos.x - x_range, self.pos.x + x_range))
    } else {
      None
    }
  }

  // shoutout to for this idea, https://github.com/WinterCore/aoc2022/blob/main/day15/main.rs#L82
  // at first i was just using a HashSet to store the ranges, but that was slow and sucked butt
  fn merge_x_covered_ranges_for_y(
    y: i64,
    ranges: &mut Vec<Sensor>,
  ) -> Vec<(i64, i64)> {
    
    let mut ranges = ranges
      .iter()
      .filter_map(|sensor| sensor.x_covered_range_for_y(y))
      .collect::<Vec<(i64, i64)>>();

    ranges.sort_by_key(|r| r.0);

    let mut result = Vec::new();

    let mut acc = ranges[0];
    for i in 1..ranges.len() {
      let range = ranges[i];
      if acc.1 >= (range.0-1) {
        acc.1 = acc.1.max(range.1);
      } else {
        result.push(acc);
        acc = range;
      }
    }

    result.push(acc);


    result
  }
}