use std::collections::HashMap;

fn check_01(i: usize) -> bool {
  let i = format!("{}", i);

  let v: Vec<_> = i.chars().collect();

  let mut s = v.clone();
  s.sort();

  if v == s {
    let mut h = HashMap::new();

    for c in v {
      *h.entry(c).or_insert(0) += 1;
    }

    h.values().any(|&v| v >= 2)
  } else {
    false
  }
}

fn solve_01(range: std::ops::RangeInclusive<usize>) -> usize {
  let mut count = 0;

  for i in range {
    if check_01(i) {
      count += 1;
    }
  }

  count
}

fn check_02(i: usize) -> bool {
  let i = format!("{}", i);

  let v: Vec<_> = i.chars().collect();

  let mut s = v.clone();
  s.sort();

  if v == s {
    let mut h = HashMap::new();

    for c in v {
      *h.entry(c).or_insert(0) += 1;
    }

    h.values().any(|&v| v == 2)
  } else {
    false
  }
}

fn solve_02(range: std::ops::RangeInclusive<usize>) -> usize {
  let mut count = 0;

  for i in range {
    if check_02(i) {
      count += 1;
    }
  }

  count
}

pub fn solve(_: &str) {
  println!("part one: {:?}", solve_01(248_345..=746_315));
  println!("part one: {:?}", solve_02(248_345..=746_315));
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn part_one() {
    assert!(check_01(111_111));
    assert!(!check_01(223_450));
    assert!(!check_01(123_789));
  }

  #[test]
  fn part_two() {
    assert!(check_02(112_233));
    assert!(!check_02(123_444));
    assert!(check_02(111_122));
  }
}
