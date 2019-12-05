use std::collections::HashMap;

fn is_sorted<T: PartialOrd>(slice: &[T]) -> bool {
  if slice.is_empty() {
    return true;
  }

  let mut last = &slice[0];

  for i in slice {
    if i < last {
      return false;
    }
    last = i;
  }

  true
}

fn digits(i: usize) -> Vec<usize> {
  let mut m = Vec::with_capacity(6);

  let mut n = i;
  while n > 0 {
    m.push(n % 10);
    n /= 10;
  }

  m.reverse();

  m
}

fn check_01(i: usize) -> bool {
  let v: Vec<_> = digits(i);

  if is_sorted(&v) {
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
  range.filter(|&i| check_01(i)).count()
}

fn check_02(i: usize) -> bool {
  let v: Vec<_> = digits(i);

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
  range.filter(|&i| check_02(i)).count()
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
