use std::cmp::{max, min};

fn part_one() {
  let coords = include_str!("./input.txt")
    .split("\n")
    .map(|l| l.split(" -> ").collect::<Vec<_>>())
    .map(|strs| {
      return strs.iter()
        .map(|s| {
          return s.split(",")
            .map(|n| u32::from_str_radix(n, 10).unwrap())
            .collect::<Vec<_>>()
        })
        .map(|x| (x[0], x[1]))
        .collect::<Vec<_>>();
    })
    .map(|points| (points[0], points[1]))
    .filter(|(p1, p2)| {
      let (x1, y1) = p1;
      let (x2, y2) = p2;

      return x1 == x2 || y1 == y2;
    })
    .collect::<Vec<_>>();

  let mut row: Vec<u32> = vec![0; 1000];
  let mut field = vec![row.clone(); 1000];

  coords.iter().for_each(|line| {
    let (p1, p2) = line;
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let x_start = *min(x1, x2);
    let x_end = *max(x1, x2);
    let y_start = *min(y1, y2);
    let y_end = *max(y1, y2);

    for x in x_start..x_end + 1 {
      for y in y_start..y_end + 1 {
        let y_coord = y as usize;
        let x_coord = x as usize;

        field[y_coord][x_coord] = field[y_coord][x_coord] + 1;
      }
    }
  });

  let overlaps = field.iter().fold(0, |sum, row| {
    let points = row.iter()
      .filter(|n| **n >= 2)
      .count();

    return sum + points;
  });

  println!("{:?}", overlaps);
}

fn is_diagonale(p1: &(u32, u32), p2: &(u32, u32)) -> bool {
  let (x1, y1) = p1;
  let (x2, y2) = p2;
  let x_start = min(x1, x2);
  let x_end = max(x1, x2);
  let y_start = min(y1, y2);
  let y_end = max(y1, y2);

  return x_end - x_start == y_end - y_start;
}

fn part_two() {
  let coords = include_str!("./input.txt")
    .split("\n")
    .map(|l| l.split(" -> ").collect::<Vec<_>>())
    .map(|strs| {
      return strs.iter()
        .map(|s| {
          return s.split(",")
            .map(|n| u32::from_str_radix(n, 10).unwrap())
            .collect::<Vec<_>>()
        })
        .map(|x| (x[0], x[1]))
        .collect::<Vec<_>>();
    })
    .map(|points| (points[0], points[1]))
    .filter(|(p1, p2)| {
      let (x1, y1) = p1;
      let (x2, y2) = p2;

      return x1 == x2 || y1 == y2 || is_diagonale(p1, p2);
    })
    .collect::<Vec<_>>();

  let mut row: Vec<u32> = vec![0; 1000];
  let mut field = vec![row.clone(); 1000];

  coords.iter().for_each(|line| {
    let (p1, p2) = line;
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let x_start = *min(x1, x2);
    let x_end = *max(x1, x2);
    let y_start = *min(y1, y2);
    let y_end = *max(y1, y2);

    if is_diagonale(p1, p2) {
      if x2 > x1 && y2 > y1 || x1 > x2 && y1 > y2 {
        let mut x = x_start;
        let mut y = y_start;

        while x <= x_end && y <= y_end {
          let y_coord = y as usize;
          let x_coord = x as usize;

          field[y_coord][x_coord] = field[y_coord][x_coord] + 1;

          x += 1;
          y += 1;
        }
      } else {
        let mut x = x_start;
        let mut y = y_end;

        while x <= x_end && y >= y_start {
          let y_coord = y as usize;
          let x_coord = x as usize;

          field[y_coord][x_coord] = field[y_coord][x_coord] + 1;

          x += 1;
          y -= 1;
        }
      }
    } else {
      for x in x_start..x_end + 1 {
        for y in y_start..y_end + 1 {
          let y_coord = y as usize;
          let x_coord = x as usize;

          field[y_coord][x_coord] = field[y_coord][x_coord] + 1;
        }
      }
    }
  });

  let overlaps = field.iter().fold(0, |sum, row| {
    let points = row.iter()
      .filter(|n| **n >= 2)
      .count();

    return sum + points;
  });

  println!("{:?}", overlaps);
}

pub fn main() {
  part_one();
  part_two();
}
