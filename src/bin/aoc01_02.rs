use std::env::current_dir;
use aoc2023::utils::read_input;

extern crate aoc2023;

fn main() {
    println!("{}", current_dir().unwrap().display());
    let lines = read_input("aoc01_02").unwrap();
    println!("Result: {}", exec(lines));
}

fn exec<T>(lines: impl Iterator<Item=T>) -> i32 where T: Into<String> {
    lines.filter_map(|line| parse_line(line.into())).sum::<i32>()
}

static PATTERNS: [(&str, &str); 9] = {
    [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
};
fn parse_line(mut line: String) -> Option<i32> {
    replace_numbers(&mut line);
    let first = line.chars().find(|char| char.is_ascii_digit())?;
    let last = line.chars().rfind(|char| char.is_ascii_digit())?;
    format!("{}{}", first, last).parse().ok()
}

fn replace_numbers(line: &mut String) {
        let mut i = 0;
        while i < line.len() {
            for (pattern, replacement) in PATTERNS {
                if i + pattern.len() <= line.len() && &line[i..i + pattern.len()] == pattern {
                    line.replace_range(i..i + pattern.len() - 1, replacement);
                    break;
                }
            }
            i += 1;
        }
}

#[cfg(test)]
mod tests {
    use crate::exec;

    #[test]
    fn it_works() {
        // Results are 29, 83, 13, 24, 42, 14, and 76.
        let input = r"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let sut = exec(input.lines());
        assert_eq!(281, sut);
    }
}
