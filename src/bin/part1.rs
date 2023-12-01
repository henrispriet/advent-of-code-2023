fn main() {
    let input = include_str!("input.txt");
    let output = part_1(input);
    println!("{output}");
}

fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|digit| digit.to_digit(10));
            let first = digits.next().unwrap_or_default();
            let last = digits.rev().next().unwrap_or(first);
            10 * first + last
        })
        .sum::<u32>()
        .to_string()
}

#[test]
fn it_works() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let expected = "142";
    let output = part_1(input);
    assert_eq!(expected, output);
}
