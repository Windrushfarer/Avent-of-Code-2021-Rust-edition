#![feature(drain_filter)]

const SIZE: usize = 5;

fn has_bingo<'a>(board: &Vec<Vec<u8>>) -> bool {
  let is_row_completed = board.iter().any(|r| r.iter().all(|n| *n == 1));
  let is_col_completed = (0..SIZE).fold(false, |completed, col| {
    if !completed {
      return board.iter().all(|row| row[col] == 1);
    }

    return completed;
  });

  return is_row_completed || is_col_completed;
}

fn part_one() {
  let (nums, boards) = include_str!("./input.txt")
    .split_once("\n\n")
    .unwrap();

  let nums: Vec<u32> = nums.split(",")
    .map(|n| n.parse().unwrap())
    .collect();

  let mut boards: Vec<Vec<Vec<u32>>> = boards.split("\n\n")
    .map(|board| {
      board
        .split("\n")
        .map(|r| {
          r.split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>()
        })
        .collect()
    }).collect::<Vec<_>>();

  let mut has_winner = false;

  let mut visited: Vec<Vec<Vec<u8>>> = (0..boards.len())
    .map(|_| {
      let checks: Vec<u8> = vec![0; SIZE];

      return vec![checks.clone(); SIZE];
    })
    .collect::<Vec<_>>();

  let score = nums.iter()
    .map_while(|num| {
      if has_winner {
        return None;
      }

      boards.iter()
        .enumerate()
        .for_each(|(i, board) | {
          board.iter()
            .enumerate()
            .for_each(|(r, row)| {
              row.iter()
                .enumerate()
                .for_each(|(c, col)| {
                  if *col == *num {
                    visited[i][r][c] = 1 as u8;
                  }
                })
            })
        });

      let winner = boards.iter()
        .enumerate()
        .filter(|(i, board)| has_bingo(&visited.get(*i).unwrap()))
        .last();

      if winner.is_some() {
        let (board_index, winner_board) = winner.unwrap();

        let score_unvisited: u32 = winner_board.iter()
          .enumerate()
          .fold(0, |sum, (r_index, row)| {
            let row_sum: u32 = row.iter()
              .enumerate()
              .fold(0, |s, (col_index, n)| {
                if visited[board_index][r_index][col_index] == 0 {
                  return s + n
                }

                return s
              });

            sum + row_sum as u32
          });

        has_winner = true;
        return Some(score_unvisited * *num)
      }

      return Some(0);
    })
    .last()
    .unwrap();

  println!("{:?}", score)
}

fn part_two() {
  let (nums, boards) = include_str!("./input.txt")
    .split_once("\n\n")
    .unwrap();

  let nums: Vec<u32> = nums.split(",")
    .map(|n| n.parse().unwrap())
    .collect();

  let mut boards: Vec<Vec<Vec<u32>>> = boards.split("\n\n")
    .map(|board| {
      board
        .split("\n")
        .map(|r| {
          r.split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>()
        })
        .collect()
    }).collect::<Vec<_>>();

  let mut winner_found = false;
  let mut first_winner: i8 = -1;

  let mut visited: Vec<Vec<Vec<u8>>> = (0..boards.len())
    .map(|_| {
      let checks: Vec<u8> = vec![0; SIZE];

      return vec![checks.clone(); SIZE];
    })
    .collect::<Vec<_>>();

  let score = nums.iter()
    .map_while(|num| {
      if winner_found {
        return None;
      }

      boards.iter()
        .enumerate()
        .for_each(|(i, board) | {
          board.iter()
            .enumerate()
            .for_each(|(r, row)| {
              row.iter()
                .enumerate()
                .for_each(|(c, col)| {
                  if *col == *num {
                    visited[i][r][c] = 1 as u8;
                  }
                })
            })
        });


      let winner = boards.iter()
        .enumerate()
        .filter(|(i, board)| {
          first_winner != *i as i8 && has_bingo(&visited.get(*i).unwrap())
        })
        .last();

      if winner.is_some() {
        let (board_index, winner_board) = winner.unwrap();

        if first_winner == -1 {
          first_winner = board_index as i8;

          return Some(0);
        }
        println!("{}", first_winner);
        let score_unvisited: u32 = winner_board.iter()
          .enumerate()
          .fold(0, |sum, (r_index, row)| {
            let row_sum: u32 = row.iter()
              .enumerate()
              .fold(0, |s, (col_index, n)| {
                if visited[board_index][r_index][col_index] == 0 {
                  return s + n
                }

                return s
              });

            sum + row_sum as u32
          });

        winner_found = true;

        return Some(score_unvisited * *num)
      }

      return Some(0);
    })
    .last()
    .unwrap();

  println!("{:?}", score)
}

pub fn main() {
  part_one();
  part_two();
}
