/**
 * --- Day 1: Sonar Sweep ---
 */
use crate::Day;
pub struct Day1;
impl Day for Day1 {
  fn part1(&self, input: &String) -> String {
    let mut prev: u32 = 9999;
    let mut count = 0;
    for line in input.split(|c| (c == '\n')) {
      if line.len() > 0 {
        let mut curr = line.parse::<u32>().unwrap();
        if prev != 9999 {
          if curr > prev {
            count += 1;
          }
        }
        prev = curr;
      }
    }
    return String::from(format!("{}", count));
  }
  fn part2(&self, input: &String) -> String {
    let mut nums = Vec::<i32>::new();
    let mut i = 0;
    let mut count = 0;
    for line in input.split(|c| (c == '\n')) {
      if line.len() > 0 {
        nums.push(line.parse().unwrap());
      }
    }

    let mut prev = 999;

    while i < nums.len() - 2 {
      let sum = nums[i] + nums[i + 1] + nums[i + 2];
      if sum > prev {
        count += 1;
      }
      prev = sum;
      i += 1;
    }
    return String::from(format!("{}", count));
  }
}
