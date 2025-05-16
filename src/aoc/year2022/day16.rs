use std::collections::{HashMap, HashSet};

pub fn main(data: &str) {
  let valves = parse_data(data);

  println!("Part 1");
  let max_flow = part1(&valves);
  println!("Max flow = {max_flow}");

  println!("Part 2");
  let max_flow = part2(&valves);
  println!("Max flow = {max_flow}");
  
}

fn part1(valves: &HashMap<String, Valve>) -> i64 {
  let mut total_flow = 0;
  let mut turned_on = HashMap::new();
  valves.values().for_each(|valve| {
    if valve.flow_rate > 0 {
      turned_on.insert(valve.name.clone(), false);
    } else {
      turned_on.insert(valve.name.clone(), true);
    }
  });
  let start = ("AA", 30, 0, 0, turned_on.clone());
  let mut queue = vec![start];
  let mut memo = HashSet::new();

  while !queue.is_empty() {
    let (valve, time, current_flow_rate, current_total_flow, turned_on) = queue.pop().unwrap();

    let valve = valves.get(valve).unwrap();

    // check if we've been here already
    if memo.contains(&(valve.name.clone(), time, current_total_flow)) {
      continue;
    } else {
      memo.insert((valve.name.clone(), time, current_total_flow));
    }

    // check if time is up
    if time <= 0 {
      total_flow = total_flow.max(current_total_flow);
      continue;
    }

    // check if all valves are turned on, meaning we can just calculate the remaining amount
    if turned_on.values().all(|&v| v) {
      total_flow = total_flow.max(current_total_flow + (time * current_flow_rate));
      continue;
    }

    if !turned_on[&valve.name] {
      let mut new_turned_on = turned_on.clone();
      new_turned_on.insert(valve.name.clone(), true);
      queue.push((&valve.name, time - 1, current_flow_rate + valve.flow_rate, current_total_flow + current_flow_rate, new_turned_on.clone()));
    }

    for neighbor in &valve.neighbors {
      queue.push((neighbor, time - 1, current_flow_rate, current_total_flow + current_flow_rate, turned_on.clone()));
    }
  }

  total_flow
}

fn part2(valves: &HashMap<String, Valve>) -> i64 {
  let mut total_flow = 0;
  let mut turned_on = HashMap::new();
  valves.values().for_each(|valve| {
    if valve.flow_rate > 0 {
      turned_on.insert(valve.name.clone(), false);
    } else {
      turned_on.insert(valve.name.clone(), true);
    }
  });
  let start = ("AA", 26, 0, 0, turned_on.clone());
  let mut queue = vec![start];
  // let mut queue2 = vec![start2];

  let mut memo = HashSet::new();
  // let mut memo2 = HashSet::new();

  while !queue.is_empty() {
    let (valve, time, current_flow_rate, current_total_flow, turned_on) = queue.pop().unwrap();

    let valve = valves.get(valve).unwrap();

    // check if we've been here already
    if memo.contains(&(valve.name.clone(), time, current_total_flow)) {
      continue;
    } else {
      memo.insert((valve.name.clone(), time, current_total_flow));
    }

    // check if time is up
    if time <= 0 {
      total_flow = total_flow.max(current_total_flow);
      continue;
    }

    // check if all valves are turned on, meaning we can just calculate the remaining amount
    if turned_on.values().all(|&v| v) {
      total_flow = total_flow.max(current_total_flow + (time * current_flow_rate));
      continue;
    }

    if !turned_on[&valve.name] {
      let mut new_turned_on = turned_on.clone();
      new_turned_on.insert(valve.name.clone(), true);
      queue.push((&valve.name, time - 1, current_flow_rate + valve.flow_rate, current_total_flow + current_flow_rate, new_turned_on.clone()));
    }

    for neighbor in &valve.neighbors {
      queue.push((neighbor, time - 1, current_flow_rate, current_total_flow + current_flow_rate, turned_on.clone()));
    }
  }
  let start = ("AA", 26, 0, 0, turned_on.clone());
  let mut queue = vec![start];
  let mut memo = HashSet::new();
  let mut total_flow2 = 0;

  while !queue.is_empty() {
    let (valve, time, current_flow_rate, current_total_flow, turned_on) = queue.pop().unwrap();

    let valve = valves.get(valve).unwrap();

    // check if we've been here already
    if memo.contains(&(valve.name.clone(), time, current_total_flow)) {
      continue;
    } else {
      memo.insert((valve.name.clone(), time, current_total_flow));
    }

    // check if time is up
    if time <= 0 {
      total_flow2 = total_flow2.max(current_total_flow);
      continue;
    }

    // check if all valves are turned on, meaning we can just calculate the remaining amount
    if turned_on.values().all(|&v| v) {
      total_flow2 = total_flow2.max(current_total_flow + (time * current_flow_rate));
      continue;
    }

    if !turned_on[&valve.name] {
      let mut new_turned_on = turned_on.clone();
      new_turned_on.insert(valve.name.clone(), true);
      queue.push((&valve.name, time - 1, current_flow_rate + valve.flow_rate, current_total_flow + current_flow_rate, new_turned_on.clone()));
    }

    for neighbor in &valve.neighbors {
      queue.push((neighbor, time - 1, current_flow_rate, current_total_flow + current_flow_rate, turned_on.clone()));
    }
  }

  total_flow + total_flow2
}

fn parse_data(data: &str) -> HashMap<String, Valve> {
  let mut valves = HashMap::new();

  data.lines().for_each(|line| {
    let valve = Valve::from(line);
    valves.insert(valve.name.clone(), valve);
  });

  valves
}

#[derive(Debug, Clone)]
struct Valve {
  name: String,
  flow_rate: i64,
  neighbors: Vec<String>,
}

impl From<&str> for Valve {
  fn from(line: &str) -> Self {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let name = parts[1].to_string();
    let flow_rate = parts[4].split('=').nth(1).unwrap().split(';').nth(0).unwrap().parse::<i64>().unwrap();
    let neighbors = parts[9..].iter().map(|s| s.to_string().trim_matches(',').to_string()).collect();
    Self { name, flow_rate, neighbors }
  }
}