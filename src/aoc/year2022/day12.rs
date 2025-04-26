use super::util::out_of_bounds;

pub fn main(data: &str) {
  let ranges = parse_data(data);

  println!("Part 1");
  let shortest_path_distance = part1(ranges.clone());
  println!("Shortest path distance: {}", shortest_path_distance);

  println!("Part 2");
  let shortest_path_distance = part2(ranges);
  println!("Shortest path distance: {}", shortest_path_distance);
}

fn part1(ranges: Vec<Vec<char>>) -> i64 {
  let mountain_range = MountainRange::new(ranges);
  mountain_range.find_shortest_path_from_s()
}

fn part2(ranges: Vec<Vec<char>>) -> i64 {
  let mountain_range = MountainRange::new(ranges);
  mountain_range.find_shortest_path_from_any_a()
}

fn parse_data(data: &str) -> Vec<Vec<char>> {
  data.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug, Clone)]
struct MountainRange {
  ranges: Vec<Vec<u64>>,
  start_s: (usize, usize),
  starting_positions: Vec<(usize, usize)>,
  end: (usize, usize),
}

impl MountainRange {
  fn new(mut ranges: Vec<Vec<char>>) -> Self {
    let start_s = ranges.iter().enumerate().find_map(|(y, row)| {
      row.iter().position(|&c| c == 'S').map(|x| (x, y))
    }).unwrap_or((0, 0));
    ranges[start_s.1][start_s.0] = 'a';

    let starting_positions = ranges.iter().enumerate().filter_map(|(y, row)| {
      row.iter().position(|&c| c == 'a').map(|x| (x, y))
    }).collect::<Vec<_>>();

    let end = ranges.iter().enumerate().find_map(|(y, row)| {
      row.iter().position(|&c| c == 'E').map(|x| (x, y))
    }).unwrap_or((0, 0));
    ranges[end.1][end.0] = 'z';
  
    MountainRange { ranges: ranges.iter().map(|row| row.iter().map(|&c| c as u64).collect()).collect(), start_s, starting_positions, end, }
  }

  fn find_shortest_path_from_s(&self) -> i64 {
    self.find_shortest_path(self.start_s)
  }

  fn find_shortest_path_from_any_a(&self) -> i64 {
    let mut shortest_path = i64::MAX;
    for start in &self.starting_positions {
      let path_distance = self.find_shortest_path(*start);
      if path_distance != -1 && path_distance < shortest_path {
        shortest_path = path_distance;
      }
    }
    shortest_path
  }
  
  fn find_shortest_path(&self, start: (usize, usize)) -> i64 {
    let mut directions = vec![
      (start.0, start.1, start.0.wrapping_add(1), start.1, 1),
      (start.0, start.1, start.0.wrapping_sub(1), start.1, 1),
      (start.0, start.1, start.0, start.1.wrapping_add(1), 1),
      (start.0, start.1, start.0, start.1.wrapping_sub(1), 1),
    ];
    let mut visited = vec![vec![false; self.ranges[0].len()]; self.ranges.len()];
    visited[start.1][start.0] = true;

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