pub fn main(data: &str) {
  let commands = parse_data(data);

  println!("Part 1");
  let part1_result = part1(commands.clone());
  println!("Sum: {}", part1_result);
}

fn part1(commands: Vec<String>) -> i32 {
  let mut cpu = CPU::new();

  commands.iter().for_each(|command_line| cpu.parse_command(command_line));

  cpu.signal_strengths.iter().sum()
}

fn parse_data(data: &str) -> Vec<String> {
    data.lines().map(|row| row.to_string()).collect()
}

struct CPU {
    cycle: i32,
    x: i32,
    signal_strengths: Vec<i32>,
}

impl CPU {
  fn new() -> Self {
    Self { cycle: 0, x: 1, signal_strengths: Vec::new() }
  }

  fn tick(&mut self, amount: i32) {
    self.cycle += amount;
    if (self.cycle - 20) % 40 == 0 {
      self.signal_strengths.push(self.x * self.cycle);
    }
  }

  fn addx(&mut self, value: i32) {
    self.tick(1);
    self.tick(1);
    self.x += value;
  }

  fn noop(&mut self) {
    self.tick(1);
  }

  fn parse_command(&mut self, command_line: &str) {
    let parts: Vec<&str> = command_line.split_whitespace().collect();
    match parts.as_slice() {
      ["noop"] => {
        self.noop();
      },
      ["addx", value] => {
        self.addx(value.parse::<i32>().unwrap());
      },
      _ => panic!("Invalid command"),
    }
  }
}