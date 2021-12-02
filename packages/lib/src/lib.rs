use std::fs::File;
use std::io::{BufReader, BufRead, Error};

pub fn read_num_input(path: &str) -> Result<Vec<i32>, Error> {
  let mut nums = vec![];
  let input = File::open(path)?;
  let buffered = BufReader::new(input);

  for line in buffered.lines() {
    nums.push(line?.parse().unwrap())
  }

  Ok(nums)
}