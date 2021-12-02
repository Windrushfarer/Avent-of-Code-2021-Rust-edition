use aoc_lib::read_num_input;
use std::path::Path;

fn count_increases(path: &str) -> i32 {
  let nums = read_num_input(path).unwrap();

  let mut prefix_sum = vec![nums[0]];

  for n in 1..nums.len() {
    prefix_sum.push(prefix_sum[n - 1] + nums[n])
  }

  let mut count = 0;
  let mut prev = prefix_sum[2];

  for n in 3..nums.len() {
    let current = prefix_sum[n] - prefix_sum[n - 3];

    if current > prev {
      count = count + 1;
    }

    prev = current
  }

  return count;
}

fn main() {
  let path = Path::new("packages/day_1/src/input.txt");
  let count = count_increases(path.to_str().unwrap());
  println!("{}", count)
}
