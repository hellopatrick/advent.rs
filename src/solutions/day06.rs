use std::collections::{HashMap, HashSet};

fn to_graph(input: &str) -> HashMap<&str, &str> {
  input
    .lines()
    .map(|line| {
      let mut parts = line.trim().split(')');

      let a = parts.next().expect("aoc input");
      let b = parts.next().expect("aoc input");

      (b, a)
    })
    .collect()
}

fn path_to_root<'a>(key: &'a str, graph: &'a HashMap<&str, &str>) -> Vec<&'a str> {
  std::iter::successors(Some(key), |k| graph.get(*k).copied()).collect()
}

fn solve_01(graph: &HashMap<&str, &str>) -> usize {
  graph
    .keys()
    .map(|key| path_to_root(key, graph).len() - 1)
    .sum()
}

fn solve_02(graph: &HashMap<&str, &str>) -> usize {
  let me = path_to_root("YOU", graph);
  let santa = path_to_root("SAN", graph);

  let me: HashSet<_> = me.iter().collect();
  let santa: HashSet<_> = santa.iter().collect();

  me.symmetric_difference(&santa).count() - 2
}

pub fn solve(input: &str) {
  let graph = to_graph(input);

  println!("part one: {}", solve_01(&graph));
  println!("part two: {}", solve_02(&graph));
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn part_one() {
    let graph = to_graph(
      "COM)B
    B)C
    C)D
    D)E
    E)F
    B)G
    G)H
    D)I
    E)J
    J)K
    K)L",
    );

    assert_eq!(solve_01(&graph), 42);
  }

  #[test]
  fn part_two() {
    let graph = to_graph(
      "COM)B
      B)C
      C)D
      D)E
      E)F
      B)G
      G)H
      D)I
      E)J
      J)K
      K)L
      K)YOU
      I)SAN",
    );

    assert_eq!(solve_02(&graph), 4);
  }
}
