const RADIX: u32 = 10;
const DIGIT_WORDS: [&str; RADIX as usize] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut digits = line.char_indices().filter_map(|(idx, char)| {
                char.to_digit(RADIX).or_else(|| {
                    DIGIT_WORDS.iter().enumerate().find_map(|(digit, &word)| {
                        line[idx..].starts_with(word).then_some(digit as u32)
                    })
                })
            });

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
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let expected = "281";
    assert_eq!(expected, process(input));
}
