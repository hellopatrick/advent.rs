extern crate advent;

use std::fs;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
  #[structopt(short)]
  day: u8,
}

fn not_implemented(_: &str) {
  println!("not implemented yet!")
}

pub fn main() {
  use advent::solutions;

  let opt = Opt::from_args();

  let day = opt.day;

  println!("day {:02}", day);

  let solve = match day {
    1 => solutions::day01::solve,
    2 => solutions::day02::solve,
    3 => solutions::day03::solve,
    4 => solutions::day04::solve,
    5 => solutions::day05::solve,
    6 => solutions::day06::solve,
    7 => solutions::day07::solve,
    8 => solutions::day08::solve,
    9 => solutions::day09::solve,
    10 => solutions::day10::solve,
    11 => solutions::day11::solve,
    _ => not_implemented,
  };

  let path = format!("./inputs/day{:02}.txt", day);
  let input = fs::read_to_string(path).expect("file not found");

  solve(&input);
}
