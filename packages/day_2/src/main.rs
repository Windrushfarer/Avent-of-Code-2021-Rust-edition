use aoc_lib::read_file_input;
use std::path::Path;

fn main() {
  let path = Path::new("packages/day_2/src/input.txt");
  let lines = read_file_input(path.to_str().unwrap()).unwrap();
  let mut steps: Vec<(&str, i32)> = vec![];

  for line in lines.iter() {
    let (dir, val) = line.split_once(" ").unwrap();

    steps.push((dir, val.parse().unwrap()));
  }

  let mut path = 0;
  let mut aim = 0;
  let mut depth = 0;

  for step in steps.iter() {
    let (direction, value) = step;

    match (*direction, value) {
      ("forward", v) => {
        path += v;
        depth += v * aim;
      },
      ("down", v) => {
        aim += v
      },
      ("up", v) => {
        aim -= v
      },
      _ => unreachable!(),
    }
  }

  let result = path * depth;

  println!("{}", result);
}
