pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

#[cfg(windows)]
const DOUBLE_LINE_ENDING: &str = "\r\n\r\n";
#[cfg(not(windows))]
const DOUBLE_LINE_ENDING: &str = "\n\n";
