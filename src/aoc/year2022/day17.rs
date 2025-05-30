use std::collections::HashSet;

pub fn main(data: &str) {
  let jet_order = parse_data(data);

  println!("Part 1");
  let height = part1(jet_order);
  println!("Height is {height}");
  
  // println!("Part 2");
}

fn part1(jet_order: Vec<char>) -> u64 {
  let shape_order = vec![
    ShapeType::FlatLine,
    ShapeType::Plus,
    ShapeType::BackwardsL,
    ShapeType::I,
    ShapeType::Square,
  ];
  let mut shape_index = 0;
  let mut jet_index = 0;
  let mut highest_height = 0;
  let mut curr_shape = Shape::new(
    shape_order[shape_index].clone(),
    (2, highest_height + 4)
  );
  let mut landed_positions = HashSet::new();
  (0..7).for_each(|x| {
    landed_positions.insert((x, 0));
  });

  (1..=2022).for_each(|_| {
    curr_shape = Shape::new(shape_order[shape_index].clone(), (2, highest_height + 4));
    loop {
      curr_shape = curr_shape.wind_blow(jet_order[jet_index], &landed_positions);
      jet_index = (jet_index + 1) % jet_order.len();
      let next_shape = curr_shape.fall();
      
      if next_shape.get_current_positions()
          .iter()
          .any(|pos| landed_positions.contains(pos))
      {
        highest_height = highest_height.max(curr_shape.get_height());
        landed_positions.extend(curr_shape.get_current_positions());
        break;
      }
      curr_shape = next_shape;
    }
    shape_index = (shape_index + 1) % shape_order.len();
  });

  highest_height as u64
}

fn parse_data(data: &str) -> Vec<char> {
  data.chars().collect()
}

#[derive(Debug, Clone)]
enum ShapeType {
  FlatLine,
  Plus,
  BackwardsL,
  I,
  Square,
}

#[derive(Debug, Clone)]
struct Shape {
  directions: Vec<(i64, i64)>,
  position: (i64, i64),
}

impl Shape {
  fn new(shape_type: ShapeType, position: (i64, i64)) -> Self {
    Self {
      directions: match shape_type {
        ShapeType::FlatLine   => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        ShapeType::Plus       => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        ShapeType::BackwardsL => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        ShapeType::I          => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        ShapeType::Square     => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
      },
      position,
    }
  }

  fn wind_blow(&self, direction: char, landed_positions: &HashSet<(i64, i64)>) -> Self {
    let mut clone = self.clone();

    let direction = if direction == '<' { -1 } else { 1 };

    if clone.directions.iter().any(
        |d| 
          (clone.position.0 + d.0) + direction < 0 || 
          (clone.position.0 + d.0) + direction > 6 || 
          landed_positions.contains(&(clone.position.0 + d.0 + direction, clone.position.1 + d.1))
    ) {
      return clone;
    }

    clone.position.0 += direction;

    clone
  }
  
  fn fall(&self) -> Self {
    let mut clone = self.clone();
    clone.position.1 -= 1;
    clone
  }

  fn get_height(&self) -> i64 {
    self.get_current_positions().iter().max_by(|a, b| a.1.cmp(&(b.1))).unwrap().1
  }

  fn get_current_positions(&self) -> Vec<(i64, i64)> {
    self.directions
      .iter()
      .map(|d| (self.position.0 + d.0, self.position.1 + d.1))
      .collect()
  }
  
  fn to_string(&self) -> String {
    self.get_current_positions()
      .iter()
      .map(|p| format!("({},{})", p.0, p.1))
      .collect::<Vec<String>>()
      .join(",")
  }
}