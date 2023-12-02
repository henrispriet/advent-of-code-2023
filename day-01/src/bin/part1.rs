const RADIX: u32 = 10;

fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|digit| digit.to_digit(RADIX));
            let first = digits.next().unwrap_or_default();
            let last = digits.last().unwrap_or(first);
            RADIX * first + last
        })
        .sum::<u32>()
        .to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    println!("{output}");
}

#[test]
fn example() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let expected = "142";
    assert_eq!(expected, process(input));
}

#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "55712";
    assert_eq!(expected, process(input));
}
