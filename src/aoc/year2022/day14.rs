pub fn main(data: &str) {
  let rock_shapes = parse_data(data);

  println!("Part 1");
  let sand_count = part1(&rock_shapes);
  println!("Sand count = {sand_count}");

  println!("Part 2");
  let sand_count = part2(&rock_shapes);
  println!("Sand count = {sand_count}");
}

fn part1(rock_shapes: &Vec<Vec<(usize, usize)>>) -> usize {
  let mut cave = Cave::new();
  for rock in rock_shapes {
    cave.add_rock(rock);
  }
  cave.fill(false)
}

fn part2(rock_shapes: &Vec<Vec<(usize, usize)>>) -> usize {
  let mut cave = Cave::new();
  for rock in rock_shapes {
    cave.add_rock(rock);
  }
  cave.fill(true)
}

fn parse_data(data: &str) -> Vec<Vec<(usize, usize)>> {
  let mut rocks = Vec::new();

  data.lines().for_each(|line| {
    let rock = line
      .split(" -> ")
      .map(|coord| {
        let (x, y) = coord.split_once(',').unwrap();
        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
      })
      .collect::<Vec<_>>();
    rocks.push(rock);
  });

  rocks
}

struct Cave {
  grid: Vec<Vec<bool>>,
  max_y: usize,
}

impl Cave {
  fn new() -> Self {
    Self {
      grid: vec![vec![false; 1000]; 1000],
      max_y: 0,
    }
  }

  fn add_rock(&mut self, rock: &[(usize, usize)]) {
    let mut curr_x = rock[0].0;
    let mut curr_y = rock[0].1;
    self.max_y = self.max_y.max(curr_y);
    for i in 1..rock.len() {
      let (next_x, next_y) = rock[i];
      if curr_x == next_x {
        for y in curr_y.min(next_y)..=curr_y.max(next_y) {
          self.grid[y][curr_x] = true;
          self.max_y = self.max_y.max(y);
        }
      } else if curr_y == next_y {
        for x in curr_x.min(next_x)..=curr_x.max(next_x) {
          self.grid[curr_y][x] = true;
        }
      } else {
        unreachable!("Invalid rock path found: {:?}", rock);
      }
      curr_x = next_x;
      curr_y = next_y;
    }
  }

  fn fill(&mut self, generate_floor: bool) -> usize {
    if generate_floor {
      for x in 0..self.grid[0].len() {
        self.grid[self.max_y + 2][x] = true;
      }
    }

    let mut sand_count = if generate_floor { 1 } else { 0 };
    while self.drop_sand() {
      sand_count += 1;
    }
    
    sand_count
  }

  fn drop_sand(&mut self) -> bool {
    let mut x = 500;
    let mut y = 0;
    while y < self.max_y + 2 {
      if !self.grid[y + 1][x] {
        y += 1;
      } else if !self.grid[y + 1][x - 1] {
        x -= 1;
        y += 1;
      } else if !self.grid[y + 1][x + 1] {
        x += 1;
        y += 1;
      } else {
        if x == 500 && y == 0 {
          return false;
        }
        self.grid[y][x] = true;
        return true;
      }
    }
    false
  }
}