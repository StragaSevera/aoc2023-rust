use std::borrow::Borrow;
use std::env::current_dir;
use std::ops::Deref;
use aoc2023::utils::read_input;

extern crate aoc2023;

fn main() {
    println!("{}", current_dir().unwrap().display());
    let lines = read_input("aoc01_01").unwrap();
    println!("Result: {}", exec(lines));
}

fn exec<T>(lines: impl Iterator<Item=T>) -> i32 where T: Deref<Target=str> {
    lines.filter_map(|line| parse_line(line.borrow())).sum::<i32>()
}

fn parse_line(line: &str) -> Option<i32> {
    let first = line.chars().find(|char| char.is_ascii_digit())?;
    let last = line.chars().rfind(|char| char.is_ascii_digit())?;
    format!("{}{}", first, last).parse().ok()
}

#[cfg(test)]
mod tests {
    use crate::exec;

    #[test]
    fn it_works() {
        let input = r"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let sut = exec(input.lines());
        assert_eq!(142, sut);
    }
}
