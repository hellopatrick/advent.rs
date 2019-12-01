extern crate advent;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
  #[structopt(short)]
  day: u8,
}

pub fn main() {
  use advent::solutions;

  let opt = Opt::from_args();
  match opt.day {
    1 => solutions::day01::solve(),
    _ => println!("not implemented yet."),
  }
}
