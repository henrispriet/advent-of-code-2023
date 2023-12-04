#[derive(Debug, Clone)]
struct Number {
    value: u32,
    first_digit_x: usize,
    last_digit_x: usize,
    // y: usize,
}

#[derive(Debug, Clone, Default)]
struct NumbersList {
    numbers: Vec<Number>,
    first_in_line: Vec<usize>,
}

fn process(input: &str) -> String {
    let mut numbers = NumbersList::default();
    let mut symbols = Vec::<(usize, usize)>::new();

    let mut new_number = true;
    for (y, line) in input.lines().enumerate() {
        numbers.first_in_line.push(numbers.numbers.len());

        for (x, char) in line.char_indices() {
            if char == '.' {
                new_number = true;
                continue;
            }

            // char != '.' => symbol or digit
            let Some(digit) = char.to_digit(10) else {
                symbols.push((x, y));
                continue;
            };

            // char is a digit
            if new_number {
                numbers.numbers.push(Number {
                    value: digit,
                    first_digit_x: x,
                    last_digit_x: x,
                    // y,
                });
                new_number = false;
            } else {
                let last_number = numbers
                    .numbers
                    .last_mut()
                    .expect("new_number == false => at least one number");
                last_number.value = last_number.value * 10 + digit;
                last_number.last_digit_x = x;
            }
        }
    }
    numbers.first_in_line.push(numbers.numbers.len());

    // NOTE: assuming a number is next to at most one symbol => map every symbol to potentially multiple numbers
    let valid_numbers = symbols.into_iter().flat_map(|(x, y)| {
        let relevant_numbers_by_line =
            // NOTE: assuming no symbols on the first or last line
            numbers.numbers[numbers.first_in_line[y - 1]..numbers.first_in_line[y + 2]].into_iter();
        relevant_numbers_by_line.filter_map(move |number| {
            (number.last_digit_x >= x - 1 && number.first_digit_x <= x + 1).then_some(number.value)
        })
    });

    // let valid_numbers = numbers.numbers.into_iter().flat_map(|number| {
    //     symbols
    //         .iter()
    //         .any(|(x, y)| {
    //             // NOTE: assuming no symbols on the first or last line
    //             number.y >= y - 1
    //                 && number.y <= y + 1
    //                 && number.last_digit_x >= x - 1
    //                 && number.first_digit_x <= x + 1
    //         })
    //         .then_some(number.value)
    // });

    valid_numbers.sum::<u32>().to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    println!("{output}");
}

#[test]
fn example() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let expected = "4361";
    assert_eq!(expected, process(input));
}

#[ignore = "not done"]
#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "";
    assert_eq!(expected, process(input));
}
