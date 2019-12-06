use std::collections::HashMap;

pub fn to_graph(input: &str) -> HashMap<&str, &str> {
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

pub fn count_orbits(key: &str, graph: &HashMap<&str, &str>) -> usize {
  if key == "COM" {
    0
  } else {
    let v = graph.get(key).unwrap();

    1 + count_orbits(*v, graph)
  }
}

pub fn solve_01(graph: &HashMap<&str, &str>) -> usize {
  graph.keys().map(|key| count_orbits(key, graph)).sum()
}

pub fn solve_02(graph: &HashMap<&str, &str>) -> usize {
  let mut ptr = *graph.get("YOU").unwrap();

  let mut path_to_com = Vec::new();

  while ptr != "COM" {
    path_to_com.push(ptr);
    ptr = *graph.get(ptr).unwrap();
  }

  let mut ptr = *graph.get("SAN").unwrap();

  let mut hops_to_path = 0;

  while !path_to_com.contains(&ptr) {
    hops_to_path += 1;
    ptr = *graph.get(ptr).unwrap();
  }

  path_to_com.iter().position(|&c| c == ptr).unwrap() + hops_to_path
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
