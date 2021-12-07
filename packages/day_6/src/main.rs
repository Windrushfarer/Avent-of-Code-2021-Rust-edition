fn calculate_population(days: u16) -> i64 {
  let mut fishes = include_str!("./input.txt")
    .split(",")
    .map(|s| u8::from_str_radix(s, 10).unwrap())
    .fold([0; 9], |mut acc, val| {
      let index = val as usize;
      acc[index] += 1;

      acc
    });

  for day in 1..days {
    fishes[((day + 7) % 9) as usize] += fishes[(day % 9) as usize]
  }

  fishes.iter().sum()
}

fn part_one() {
  println!("{}", calculate_population(80))
}

fn part_two() {
  println!("{}", calculate_population(256))
}

pub fn main() {
  part_one();
  part_two();
}
