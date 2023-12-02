use itertools::Itertools;

fn process(input: &str) -> String {
    let cube_counts = maplit::btreemap! {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
    };

    input
        .lines()
        .map(|line| {
            let game = line
                .split(": ")
                .last()
                .expect("split contains at least one element");
            game.split("; ")
                .flat_map(|round| round.split(", "))
                .all(|grab| {
                    let (num, color) = grab.split(' ').next_tuple().expect("grab has 2 elements");
                    let num = num.parse::<u32>().expect("num is number");
                    num <= cube_counts[color]
                })
        })
        .enumerate()
        .filter_map(|(idx, valid)| valid.then_some(idx + 1))
        .sum::<usize>()
        .to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    println!("{output}");
}

#[test]
fn example() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let expected = "8";
    assert_eq!(expected, process(input));
}

#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "2551";
    assert_eq!(expected, process(input));
}
