pub fn main(data: &str) {
  let monkeys = parse_data(data);
  
  // println!("Monkeys: {:#?}", monkeys);

  println!("Part 1");
  let part1_result = part1(monkeys.clone());
  println!("Top 2 Important Monkeys Multiplied: {}", part1_result);
}

fn part1(mut monkeys: Vec<Monkey>) -> i64 {
  let rounds = 20;

  for _ in 0..rounds {
    for i in 0..monkeys.len() {
      while !monkeys[i].items.is_empty() {
        let (item, next_monkey) = monkeys[i].inspect();
        monkeys[next_monkey].items.push(item);
      }
    }
  }

  let mut inspections = monkeys.iter().map(|monkey| monkey.total_inspections).collect::<Vec<i64>>();
  inspections.sort_unstable();
  inspections.reverse();
  inspections[0] * inspections[1]
}

fn parse_data(data: &str) -> Vec<Monkey> {
  data.split("\n\n")
    .map(|monkey| {
      let lines = monkey.lines().collect::<Vec<&str>>();
      let items = lines[1].split(": ").nth(1).unwrap()
        .split(", ").map(|item| item.parse::<i64>().unwrap()).collect::<Vec<i64>>();
      let operation = lines[2].split("new = old ").nth(1).unwrap().chars().next().unwrap();
      let operation_value = lines[2].split("new = old ").nth(1).unwrap().split_whitespace().nth(1).and_then(|v| if v == "old" { None } else { Some(v.parse::<i64>().unwrap()) });
      let divisor_test = lines[3].split("divisible by ").nth(1).unwrap().parse::<i64>().unwrap();
      let if_true = lines[4].split("monkey ").nth(1).unwrap().parse::<usize>().unwrap();
      let if_false = lines[5].split("monkey ").nth(1).unwrap().parse::<usize>().unwrap();

      Monkey::new(items, operation, operation_value, divisor_test, if_true, if_false)
    })
    .collect()
}

#[derive(Debug, Clone)]
struct Monkey {
  items: Vec<i64>,
  operation: char,
  operation_value: Option<i64>,
  divisor_test: i64,
  if_true: usize,
  if_false: usize, 
  total_inspections: i64,
}

impl Monkey {
  fn new(items: Vec<i64>, operation: char, operation_value: Option<i64>, divisor_test: i64, if_true: usize, if_false: usize) -> Self {
    Self { items, operation, operation_value, divisor_test, if_true, if_false, total_inspections: 0 }
  }

  fn inspect(&mut self) -> (i64, usize) {
    self.total_inspections += 1;
    let item = self.items.remove(0);
    let new_item = match self.operation {
      '+' => item + self.operation_value.unwrap_or(item),
      '*' => item * self.operation_value.unwrap_or(item),
      _ => panic!("Unknown operation"),
    } / 3;
    let next_monkey = if new_item % self.divisor_test == 0 { self.if_true } else { self.if_false };
    (new_item, next_monkey)
  }
}
