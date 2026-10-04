pub fn main(data: &str) {
  let coordinates = parse_input(data);
  
  println!("Part 1 -- Find Total Surface Area");
  let part1_result = part1(&coordinates);
  println!("Part 1 Result: {}", part1_result);
}

fn part1(cube_positions: &Vec<CubePosition>) -> u32 {
  cube_positions
    .iter()
    .map(|position| {
      6 - cube_positions
        .iter()
        .map(|cube_position| position.is_adjacent(cube_position))
        .filter(|s| *s)
        .count() as u32
    })
    .sum()
}

fn parse_input(data: &str) -> Vec<CubePosition> {
  data
    .lines()
    .map(|line| {
      let coordinates: Vec<u16> = line.split(',').map(|cell| cell.parse::<u16>().unwrap()).collect();
      if let [x,y,z] = coordinates[..] {
        CubePosition::new(x,y,z)
      } else {
        unreachable!()
      }
    })
    .collect()
}

#[derive(Debug)]
struct CubePosition {
  x: u16,
  y: u16,
  z: u16,
}
/**

X # #
X # #
# # #
*/
impl CubePosition {
  fn new(x: u16, y: u16, z: u16) -> CubePosition {
    CubePosition { x, y, z }
  }
  
  fn is_adjacent(&self, other: &CubePosition) -> bool {
    let differences = [self.x.abs_diff(other.x), self.y.abs_diff(other.y), self.z.abs_diff(other.z)];
    match differences {
      [1, 0, 0] | [0, 1, 0] | [0, 0, 1] => true,
      _ => false,
    }
  }
}
