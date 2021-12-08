fn part_one() {
  let mut subs = include_str!("./input.txt")
    .split(',')
    .map(|n| n.parse().unwrap())
    .collect::<Vec<i32>>();

  let mid = subs.len() / 2;
  let median_value = *subs.select_nth_unstable(mid).1;

  println!(
    "{}",
    // Median sum
    subs.iter().map(|n| (n - median_value).abs()).sum::<i32>()
  );
}

fn part_two() {
  let nums = include_str!("./input.txt")
    .split(',')
    .map(|n| n.parse().unwrap())
    .collect::<Vec<i32>>();
  let max = nums.iter().max().unwrap();

  let min = (0..*max)
    .map(|step| {
      nums
        .iter()
        .map(|n| {
          let distance = (n - step).abs();

          distance * (distance + 1) / 2
        })
        .sum::<i32>()
    })
    .min()
    .unwrap();

  println!("{}", min);
}

pub fn main() {
  part_one();
  part_two();
}
