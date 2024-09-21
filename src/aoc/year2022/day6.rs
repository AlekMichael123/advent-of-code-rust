pub fn main(data: &str) {
  println!("Part 1 -- Find beginning of message [size = 4]\ni = {}", part1(data));
  println!("Part 2 -- Find beginning of message [size = 14]\ni = {}", part2(data));
}

fn part1(data: &str) -> usize {
  perform(data, 4)
}

fn part2(data: &str) -> usize {
  perform(data, 14)
}

fn perform(data: &str, size: usize) -> usize {
  fn rotate_candidates(latest: char, candidates: &mut Vec<char>) {
    let mut carry = latest;
    candidates
      .iter_mut()
      .rev()
      .for_each(|c| {
        let temp = c.clone();
        *c = carry;
        carry = temp;
      });
  }

  fn check_candidates(candidates: &Vec<char>, size: usize) -> bool {
    for c in candidates {
      if candidates.iter().filter(|cand| *cand != c).count() != size - 1 {
        return false;
      }
    }
    true
  }

  let mut candidates: Vec<char> = vec![char::default(); size];
  data
    .char_indices()
    .find(|(i, c)| {
      if (0..=size-1).contains(i) {
        candidates[*i] = *c;
        false
      } else if check_candidates(&candidates, size) {
        true
      } else {
        rotate_candidates(*c, &mut candidates);
        false
      }
    })
    .unwrap()
    .0
}