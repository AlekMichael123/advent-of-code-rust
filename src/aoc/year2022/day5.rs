pub fn main(data: &str) {
  let (stacks, instructions) = match parse_input(data) {
    Ok(data) => data,
    Err(err) => panic!("{err}"),
  };
  println!(
    "Part 1 -- Perform move instructions on crate stacks\nResulting top-most crates = {}", 
    part1(&mut stacks.clone(), &instructions));

  println!(
    "Part 1 -- Perform move instructions on crate stacks while maintaing their order\nResulting top-most crates = {}", 
    part2(&mut stacks.clone(), &instructions));
}

fn part1(stacks: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>) -> String {
  instructions
    .iter()
    .for_each(|instruction| {
      let (amt, src, dest) = *instruction;
      let mut cache: Vec<char> = vec![];
      for _ in 0..amt {
        let Some(top) = stacks[src-1].pop() else {
          break;
        };
        cache.push(top);
      }
      cache
        .iter()
        .for_each(|e| stacks[dest-1].push(*e));
    });
  
  stacks
    .iter_mut()
    .filter_map(|stack| stack.pop())
    .fold("".to_string(), |acc, top| format!("{}{}", acc, top))
}

fn part2(stacks: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>) -> String {
  instructions
    .iter()
    .for_each(|instruction| {
      let (amt, src, dest) = *instruction;
      let mut cache: Vec<char> = vec![];
      for _ in 0..amt {
        let Some(top) = stacks[src-1].pop() else {
          break;
        };
        cache.push(top);
      }
      cache
        .iter()
        .rev()
        .for_each(|e| stacks[dest-1].push(*e));
    });
  
  stacks
    .iter_mut()
    .filter_map(|stack| stack.pop())
    .fold("".to_string(), |acc, top| format!("{}{}", acc, top))
}

fn parse_input(data: &str) -> Result<(Vec<Vec<char>>, Vec<Instruction>), String> {
  let mut data = data.split("\n\n");
  let (Some(data_stacks), Some(data_instructions), None) = (data.next(), data.next(), data.next()) 
  else {
    return Err("Error @ parse_input: data is in improper format.".to_string());
  };

  Ok((helper_parse_input_stacks(data_stacks), helper_parse_input_instructions(data_instructions)))
}

fn helper_parse_input_stacks(data_stacks: &str) -> Vec<Vec<char>> { 
  let mut data_stacks = data_stacks.lines().rev();
  let stacks_len: Vec<&str> = data_stacks.next().unwrap().split_ascii_whitespace().collect();
  let stacks_len = stacks_len.len();
  let mut stacks: Vec<Vec<char>> = vec![vec![]; stacks_len];

  while let Some(crates) = data_stacks.next() {
    let crates: Vec<char> = crates.as_bytes().iter().map(|c| *c as char).collect();
    let n = crates.len();
    for i in (0..n).step_by(4) {
      if crates[i] != '[' {
        continue;
      }
      stacks[i / 4].push(crates[i + 1]);
    }
  }
  stacks
}

fn helper_parse_input_instructions(data_instructions: &str) -> Vec<Instruction> { 
  data_instructions
    .lines()
    .map(|line| {
      let res: Vec<usize> = 
        line
          .split_ascii_whitespace()
          .filter_map(|e| {
            if let Ok(res) = e.parse() {
              Some(res)
            } else {
              None
            }
          })
          .collect();
      assert_eq!(res.len(), 3);
      (res[0], res[1], res[2])
    })
    .collect()
}

type Instruction = (usize, usize, usize);