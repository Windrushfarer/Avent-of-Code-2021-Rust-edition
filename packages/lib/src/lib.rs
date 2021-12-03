use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn read_num_input(path: &str) -> Result<Vec<i32>, Error> {
  let mut nums = vec![];
  let input = File::open(path)?;
  let buffered = BufReader::new(input);

  for line in buffered.lines() {
    nums.push(line?.parse().unwrap())
  }

  Ok(nums)
}

pub fn read_file_input(path: &str) -> Result<Vec<String>, Error> {
  let mut lines = vec![];
  let input = File::open(path)?;
  let buffered = BufReader::new(input);

  for line in buffered.lines() {
    lines.push(line.unwrap())
  }

  Ok(lines)
}
