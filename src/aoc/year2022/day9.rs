use std::collections::HashMap;

pub fn main(data: &str) {
  let directions = parse_data(data);

  println!("Part 1");
  perform_directions(2, &directions);

  println!("Part 2");
  perform_directions(10, &directions);
}

fn perform_directions(rope_length: usize, directions: &Vec<Vec<&str>>) {
  let mut rope = Rope::new(rope_length);
  for direction in directions.iter() {
    let (dir, steps) = (direction[0], direction[1].parse::<i32>().unwrap());
    for _ in 0..steps {
      rope.move_head(dir);
    }
  }
  println!("Visited positions: {}", rope.visited.len());
}

fn parse_data(data: &str) -> Vec<Vec<&str>> {
  data.lines().map(|row| row.split(" ").collect()).collect()
}

#[derive(Debug)]
struct Rope {
  knots: Vec<(i32, i32)>,
  visited: HashMap<(i32, i32), usize>,
}

impl Rope {
  fn new(length: usize) -> Self {
    Self {
      knots: vec![(0, 0); length],
      visited: HashMap::new(),
    }
  }

  fn move_head(&mut self, direction: &str) {
    let (x, y) = self.knots[0];
    match direction {
      "U" => self.knots[0] = (x, y + 1),
      "D" => self.knots[0] = (x, y - 1),
      "L" => self.knots[0] = (x - 1, y),
      "R" => self.knots[0] = (x + 1, y),
      _ => panic!("Invalid direction"),
    }

    for i in 1..self.knots.len() {
      let (prev_x, prev_y) = self.knots[i - 1];
      let (curr_x, curr_y) = self.knots[i];

      let dx = prev_x - curr_x;
      let dy = prev_y - curr_y;

      if dx.abs() > 1 || dy.abs() > 1 {
        self.knots[i] = (
          curr_x + dx.signum(),
          curr_y + dy.signum(),
        );
      }
      if i == self.knots.len() - 1 {
        // Track the positions visited by the last knot
        *self.visited.entry(self.knots[i]).or_insert(0) += 1;
      }
    }
  }
}