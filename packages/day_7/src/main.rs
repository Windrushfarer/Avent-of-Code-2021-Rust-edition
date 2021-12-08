fn part_one() {
  let mut subs = include_str!("./input.txt")
    .split(',')
    .map(|n| n.parse().unwrap())
    .collect::<Vec<i32>>();

  let mid = subs.len() / 2;
  let median_value = *subs.select_nth_unstable(mid).1;

  println!(
    "{}",
    subs.iter().map(|n| (n - median_value).abs()).sum::<i32>()
  );
}

pub fn main() {
  part_one()
}
