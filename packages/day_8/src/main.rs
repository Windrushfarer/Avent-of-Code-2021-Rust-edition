use std::collections::{HashMap, HashSet};

fn parse_input() -> Vec<Vec<Vec<&'static str>>> {
  return include_str!("./input.txt")
    .split("\n")
    .map(|line| {
      line
        .split(" | ")
        .map(|c| c.split(" ").collect::<Vec<_>>())
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();
}

fn part_one() {
  let data = parse_input();
  let result = data.iter().fold(0, |mut sum, entry| {
    let output = &entry[1];

    for el in output.iter() {
      sum = match el.len() {
        2 | 3 | 4 | 7 => {
          sum + 1
        }
        _ => sum
      }
    }

    sum
  });

  println!(
    "{}",
    result
  );
}

fn overlaps<'a>(first: &'a str, second: &'a str) -> bool {
  first.chars().all(|c| second.contains(c))
}

fn get_sorted_key(key: &str) -> String {
  let mut chars = key.split("").collect::<Vec<_>>();
  chars.sort();

  chars.join("")
}

fn create_digits_map(patterns: &Vec<&str>) -> HashMap<String, u8> {
  let one = patterns.iter().find(|p| p.len() == 2).unwrap();
  let seven = patterns.iter().find(|p| p.len() == 3).unwrap();
  let four = patterns.iter().find(|p| p.len() == 4).unwrap();
  let eight = patterns.iter().find(|p| p.len() == 7).unwrap();
  let three = patterns.iter().find(|p| p.len() == 5 && overlaps(*one, *p)).unwrap();
  let nine = patterns.iter().find(|p| p.len() == 6 && overlaps(*three, *p)).unwrap();
  let zero = patterns.iter().find(|p| p.len() == 6 && *p != nine && overlaps(*seven, *p)).unwrap();
  let six = patterns.iter().find(|p| p.len() == 6 && *p != nine && *p != zero).unwrap();
  let five = patterns.iter().find(|p| p.len() == 5 && overlaps(*p, *six)).unwrap();
  let two = patterns.iter().find(|p| p.len() == 5 && *p != five && *p != three).unwrap();

  HashMap::from([
    (*one, 1),
    (*two, 2),
    (*three, 3),
    (*four, 4),
    (*five, 5),
    (*six, 6),
    (*seven, 7),
    (*eight, 8),
    (*nine, 9),
    (*zero, 0),
  ].map(|(p, v)| {
    (get_sorted_key(p), v)
  }))
}

fn part_two() {
  let data = parse_input();

  let result = data.iter().map(|row| {
    let map = create_digits_map(&row[0]);
    let value = &row[1]
      .iter()
      .map(|out| {
        let key = get_sorted_key(out);
        map.get(&key).unwrap().to_string()
      })
      .collect::<Vec<_>>();

    value.join("").parse::<u32>().unwrap()
  }).sum::<u32>();

  println!("{:?}", result)
}

pub fn main() {
  part_one();
  part_two();
}

