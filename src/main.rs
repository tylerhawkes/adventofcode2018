
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod util;

fn main() {
  day1::compute(&util::file_lines("inputs/day1.txt"));
  day2::compute(&util::file_lines("inputs/day2.txt"));
  day3::compute(&util::file_lines("inputs/day3.txt"));
  day4::compute(&util::file_lines("inputs/day4.txt"));
  day5::compute(&util::file_lines("inputs/day5.txt"));
  day6::compute(&util::file_lines("inputs/day6.txt"));
}
