fn main() {
    let input = include_str!("input.txt");
    let output = part_1(input);
    println!("{output}");
}

const DIGIT_WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut digits = Vec::new();
            'chars: for (i, c) in line.char_indices() {
                if let Some(d) = c.to_digit(10) {
                    digits.push(d);
                    continue;
                }

                for (d, &word) in DIGIT_WORDS.iter().enumerate() {
                    let Some(slice) = line.get(i..i + word.len()) else {
                        continue;
                    };
                    if slice == word {
                        digits.push(d as u32);
                        // no digit word is a prefix of another digit word
                        continue 'chars;
                    }
                }
            }
            let mut digits = digits.into_iter();

            let first = digits.next().unwrap_or_default();
            let last = digits.rev().next().unwrap_or(first);
            10 * first + last
        })
        .sum::<u32>()
        .to_string()
}

#[test]
fn it_works() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let expected = "281";
    let output = part_1(input);
    assert_eq!(expected, output);
}
