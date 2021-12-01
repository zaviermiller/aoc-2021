mod day1;
pub trait Day {
  fn part1(&self, input: &String) -> String;
  fn part2(&self, input: &String) -> String;
}

pub fn days() -> Vec<Box<impl Day>> {
  return Vec::from([Box::new(day1::Day1 {})]);
}
