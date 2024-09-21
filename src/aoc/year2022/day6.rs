use std::collections::HashSet;

pub fn main(data: &str) {
  println!("Part 1 -- Find beginning of signal -- i = {}", part1(data));
}


fn part1(data: &str) -> usize {
  fn rotate_candidates(latest: char, candidates: &mut [char; 4]) {
    let mut carry = latest;
    candidates
      .iter_mut()
      .for_each(|c| {
        let temp = c.clone();
        *c = carry;
        carry = temp;
      });
  }

  fn check_candidates(candidates: [char; 4]) -> bool {
    let mut set: HashSet<char> = HashSet::new();
    candidates.iter().for_each(|c| { set.insert(*c); });
    println!("{:?} {:?}", candidates, set);
    set.len() == 4
  }

  let mut candidates = [' '; 4];
  data
    .char_indices()
    .find(|(i, c)| {
      if (0..=3).contains(i) {
        candidates[*i] = *c;
        false
      } else if check_candidates(candidates) {
        true
      } else {
        rotate_candidates(*c, &mut candidates);
        false
      }
    })
    .unwrap()
    .0
}