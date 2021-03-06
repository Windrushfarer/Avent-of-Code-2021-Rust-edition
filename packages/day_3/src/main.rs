#![feature(drain_filter)]

const WIDTH: usize = 12;
const COUNT: usize = 1000;

pub fn part_one() {
  let gamma = include_str!("./input.txt")
    .lines()
    .map(|l| usize::from_str_radix(l, 2).unwrap())
    .fold(vec![0; WIDTH], |count, bits| {
      println!("{}", bits);
      count
        .into_iter()
        .enumerate()
        .map(|(i, n)| n + ((bits & 1 << i) >> i))
        .collect()
    })
    .into_iter()
    .enumerate()
    .map(|(i, b)| ((b >= COUNT / 2) as u32) << i)
    .sum::<u32>();

  println!("{}", gamma * (!gamma & ((1 << WIDTH) - 1)));
}

pub fn part_two() {
  let nums = include_str!("./input.txt")
    .lines()
    .map(|l| u32::from_str_radix(l, 2).unwrap())
    .collect::<Vec<_>>();

  let oxy = (0..WIDTH)
    .rev()
    .scan(nums.clone(), |oxy, i| {
      let one = oxy.iter().filter(|n| *n & 1 << i > 0).count() >= (oxy.len() + 1) / 2;
      oxy.drain_filter(|n| (*n & 1 << i > 0) != one);

      return oxy.first().copied()
    })
    .last()
    .unwrap();

  let co2 = (0..WIDTH)
    .rev()
    .scan(nums, |co2, i| {
      let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
      co2.drain_filter(|n| (*n & 1 << i > 0) == one);

      return co2.first().copied()
    })
    .last()
    .unwrap();

  println!("{}", oxy * co2);
}

pub fn main() {
  part_two()
}
