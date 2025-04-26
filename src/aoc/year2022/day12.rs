use super::util::out_of_bounds;
use std::collections::VecDeque;

pub fn main(data: &str) {
  let ranges = parse_data(data);

  println!("Part 1");
  let shortest_path_distance = part1(ranges.clone());
  println!("Shortest path distance: {}", shortest_path_distance);
}

fn part1(ranges: Vec<Vec<char>>) -> i64 {
  let mountain_range = MountainRange::new(ranges);
  mountain_range.find_shortest_path()
}

fn parse_data(data: &str) -> Vec<Vec<char>> {
  data.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug, Clone)]
struct MountainRange {
  ranges: Vec<Vec<u64>>,
  start: (usize, usize),
  end: (usize, usize),
}

impl MountainRange {
  fn new(mut ranges: Vec<Vec<char>>) -> Self {
    let start = ranges.iter().enumerate().find_map(|(y, row)| {
      row.iter().position(|&c| c == 'S').map(|x| (x, y))
    }).unwrap_or((0, 0));
    ranges[start.1][start.0] = 'a';
    let end = ranges.iter().enumerate().find_map(|(y, row)| {
      row.iter().position(|&c| c == 'E').map(|x| (x, y))
    }).unwrap_or((0, 0));
    ranges[end.1][end.0] = 'z';
    println!("start: {:?}", start);
    println!("end: {:?}", end);
    MountainRange { ranges: ranges.iter().map(|row| row.iter().map(|&c| c as u64).collect()).collect(), start: start, end: end, }
  }

  fn find_shortest_path(&self) -> i64 {
    let mut directions = vec![
      (self.start.0, self.start.1, self.start.0.wrapping_add(1), self.start.1, 1),
      (self.start.0, self.start.1, self.start.0.wrapping_sub(1), self.start.1, 1),
      (self.start.0, self.start.1, self.start.0, self.start.1.wrapping_add(1), 1),
      (self.start.0, self.start.1, self.start.0, self.start.1.wrapping_sub(1), 1),
    ];
    let mut visited = vec![vec![false; self.ranges[0].len()]; self.ranges.len()];
    visited[self.start.1][self.start.0] = true;

    while !directions.is_empty() {
      let (from_x, from_y, curr_x, curr_y, distance) = directions.pop().unwrap();

      if 
        out_of_bounds(curr_x, curr_y, self.ranges[0].len(), self.ranges.len()) || 
        self.ranges[curr_y][curr_x] > self.ranges[from_y][from_x] + 1 ||
        visited[curr_y][curr_x] 
      {
        continue;
      }
      
      visited[curr_y][curr_x] = true;

      if curr_x == self.end.0 && curr_y == self.end.1 {
        return distance;
      }
      
      let next_directions = vec![
        (curr_x.wrapping_add(1), curr_y),
        (curr_x.wrapping_sub(1), curr_y),
        (curr_x, curr_y.wrapping_add(1)),
        (curr_x, curr_y.wrapping_sub(1)),
      ];

      next_directions.iter().for_each(|&(next_x, next_y)| directions.insert(0, (curr_x, curr_y, next_x, next_y, distance + 1)));
    }

    -1
  }
}