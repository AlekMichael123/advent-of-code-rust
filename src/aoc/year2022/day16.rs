use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

/*
SLOW AF
*/

pub fn main(_data: &str) {
  let valves = parse_data(_data);

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

  while let Some((valve, time, current_flow_rate, current_total_flow, turned_on)) = queue.pop() {
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
  let useful_valves = valves
    .values()
    .filter(|v| v.flow_rate > 0)
    .map(|v| v.name.clone())
    .collect::<Vec<String>>();
  
  let mut distances = HashMap::new();
  valves.keys().for_each(|from| {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([(from.clone(), 0)]);
    while let Some((curr, distance)) = queue.pop_front() {
      if !visited.insert(curr.clone()) {
        continue;
      }
      distances.insert((from.clone(), curr.clone()), distance);
      valves[&curr].neighbors.iter().for_each(|neighbor| queue.push_back((neighbor.clone(), distance + 1)));
    }
  });
  
  let mut subset_pressures = HashMap::new();
  let n = useful_valves.len();

  (0..(1 << n)).for_each(|mask| {
    let mut current_valves = BTreeSet::new();
    (0..n).for_each(|i| {
      if mask & (1 << i) != 0 {
        current_valves.insert(useful_valves[i].clone());
      }
    });
    
    let mut queue = vec![("AA".to_string(), 26, 0, 0, current_valves.clone())];
    let mut memo = HashMap::new();
    let mut best_total_flow = 0;
    while let Some((valve, time, current_flow_rate, current_total_flow, unused_valves)) = queue.pop() {
      let key = (valve.clone(), time, unused_valves.clone());
      if let Some(&memo_result) = memo.get(&key) {
        if memo_result >= current_total_flow {
          continue;
        }
      }
      memo.insert(key, current_total_flow);
      best_total_flow = best_total_flow.max(current_total_flow);
      
      unused_valves.iter().for_each(|next| {
        if let Some(&distance) = distances.get(&(valve.clone(), next.clone())) {
          let remaining_time = time - distance - 1;
          if remaining_time < 0 { return; }
          
          let mut next_unused_valves = unused_valves.clone();
          next_unused_valves.remove(next);
          let pressure = valves[next].flow_rate * remaining_time;
          queue.push((
            next.clone(),
            remaining_time,
            current_flow_rate + valves[next].flow_rate,
            current_total_flow + pressure,
            next_unused_valves,
          ));
        }
      });
    }
    subset_pressures.insert(current_valves, best_total_flow);
  });
  
  let mut max_total_flow = 0;
  subset_pressures.iter().for_each(|(my_valves, my_total_flow)| {
    let mut elephant_valves = BTreeSet::new();
    useful_valves.iter().for_each(|valve| {
      if !my_valves.contains(valve) {
        elephant_valves.insert(valve.clone());
      }
    });
    if let Some(&elephant_total_flow) = subset_pressures.get(&elephant_valves) {
      max_total_flow = max_total_flow.max(my_total_flow + elephant_total_flow);
    }
  });
  
  max_total_flow
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