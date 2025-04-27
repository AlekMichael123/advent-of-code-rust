pub fn main(data: &str) {
  let lines = parse_data(data);

  println!("Part 1");
  let sum = part1(&lines);
  println!("Sum = {sum}");

  println!("Part 2");
  let distress_signal = part2(&flatten(&lines));
  println!("Distress signal = {distress_signal}");
}

fn part1(lines: &Vec<(ValueLine, ValueLine)>) -> u64 {
  let mut score = 0;
  for (i, (left, right)) in lines.iter().enumerate() {
    if left.compare(right) {
      // println!("Pair {} is in the correct order: {} vs {}", i+1, left.to_string(), right.to_string());
      score += i as u64 + 1;
    }
  }
  score
}

fn part2(frequencies: &Vec<ValueLine>) -> u64 {
  let mut all_lines = frequencies.clone();
  let divider1 = ValueLine::from("[[2]]");
  let divider2 = ValueLine::from("[[6]]");
  all_lines.push(divider1.clone());
  all_lines.push(divider2.clone());
  all_lines.sort();

  let index1 = all_lines.iter().position(|value_line| value_line == &divider1).unwrap() + 1;
  let index2 = all_lines.iter().position(|value_line| value_line == &divider2).unwrap() + 1;
  index1 as u64 * index2 as u64
}

fn flatten(lines: &Vec<(ValueLine, ValueLine)>) -> Vec<ValueLine> {
  let mut all_lines = Vec::new();
  for (left, right) in lines.iter() {
    all_lines.push(left.clone());
    all_lines.push(right.clone());
  }
  all_lines
}

fn parse_data(data: &str) -> Vec<(ValueLine, ValueLine)> {
  let mut lines = Vec::new();
  let mut current_line = String::new();
  for line in data.lines() {
    if line.is_empty() {
      continue;
    }
    if current_line.is_empty() {
      current_line = line.to_string();
    } else {
      let left = ValueLine::from(current_line.as_str());
      let right = ValueLine::from(line);
      lines.push((left, right));
      current_line.clear();
    }
  }
  lines
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ValueLineType {
  Integer(u64),
  List(ValueLine),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValueLine {
  values: Vec<ValueLineType>,
}

impl From<&str> for ValueLine {
  fn from(s: &str) -> Self {
    use ValueLineType::*;

    let mut stack: Vec<ValueLine> = vec![ValueLine::new()];
    let mut current_number = String::new();

    for c in s.chars() {
      match c {
        '[' => stack.push(ValueLine::new()),
        ']' => {
          if !current_number.is_empty() {
            let value = current_number.parse::<u64>().unwrap();
            stack.last_mut().unwrap().values.push(Integer(value));
            current_number.clear();
          }
          let completed = stack.pop().unwrap();
          stack.last_mut().unwrap().values.push(List(completed));
        },
        ',' => {
          if !current_number.is_empty() {
            let value = current_number.parse::<u64>().unwrap();
            stack.last_mut().unwrap().values.push(Integer(value));
            current_number.clear();
          }
        },
        _ if c.is_digit(10) => {
          current_number.push(c);
        },
        _ => unreachable!(),
      }
    }

    stack.pop().unwrap()
  }
}

impl Ord for ValueLine {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    if self.compare(other) {
      std::cmp::Ordering::Less
    } else {
      std::cmp::Ordering::Greater
    }
  }
}
impl PartialOrd for ValueLine {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl ValueLine {
  fn new() -> Self {
    Self { values: Vec::new() }
  }

  // true if self/other are in the correct order
  fn compare(&self, other: &Self) -> bool {
    use ValueLineType::*;

    let mut self_iter = self.values.clone();
    let mut other_iter = other.values.clone();
    while !self_iter.is_empty() && !other_iter.is_empty() {
      let left = self_iter.remove(0);
      let right = other_iter.remove(0);

      match (left, right) {
        (Integer(left_integer), Integer(right_integer)) => {
          if left_integer < right_integer {
            return true;
          } else if left_integer > right_integer {
            return false;
          }
        },
        (List(left_list), List(right_list)) => {
          if left_list.compare(&right_list) {
            return true;
          } else if right_list.compare(&left_list) {
            return false;
          }
        },
        (Integer(left_integer), List(right_list)) => {
          let mut left_list = ValueLine::new();
          left_list.values.push(Integer(left_integer));
         
          if left_list.compare(&right_list) {
            return true;
          } else if right_list.compare(&left_list) {
            return false;
          }
        },
        (List(left_list), Integer(right_integer)) => {
          let mut right_list = ValueLine::new();
          right_list.values.push(Integer(right_integer));
         
          if left_list.compare(&right_list) {
            return true;
          } else if right_list.compare(&left_list) {
            return false;
          }
        },
      }
    }

    if other_iter.is_empty() { 
      false 
    } else {
      true 
    } 
  }

  fn to_string(&self) -> String {
    use ValueLineType::*;

    let mut result = String::new();
    result.push('[');
    for (i, value) in self.values.iter().enumerate() {
      if i > 0 {
        result.push(',');
      }
      match value {
        Integer(v) => result.push_str(&v.to_string()),
        List(list) => result.push_str(&list.to_string()),
      }
    }
    result.push(']');
    result
  }
}