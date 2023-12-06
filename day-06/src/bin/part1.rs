use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u32, space0},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    Parser, combinator::opt,
};
use nom_supreme::ParserExt;

fn process(_input: BoatRaces) -> String {
    todo!()
}

#[derive(Debug)]
struct BoatRaces {
    records: Vec<u32>,
    distances: Vec<u32>,
}

fn parse(input: &str) -> BoatRaces {
    let list = |name: &'static str| tag(name).precedes(space0).precedes(separated_list1(space1::<&str, ()>, u32));
    let mut parser = separated_pair(list("Time:"), line_ending, list("Distance:")).terminated(opt(line_ending))
        .map(|(records, distances)| BoatRaces{ records, distances });

    match parser.parse(input) {
        Ok(("", output)) => output,
        Ok(output) => panic!(
            "parsing INCOMPLETE!
{output:#?}"
        ),
        Err(error) => panic!(
            "parser FAILED!
{error:#?}"
        ),
    }
}

fn main() {
    let input = include_str!("input.txt");

    let parsed = parse(input);
    let output = process(parsed);
    println!("{output}");
}

#[test]
fn parse_example() {
    let example_input = "Time:      7  15   30
Distance:  9  40  200";
    parse(example_input);
}

#[test]
fn parse_real_input() {
    let real_input = include_str!("input.txt");
    parse(real_input);
}

#[ignore = "not done with parse"]
#[test]
fn example() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    let expected = "288";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}

#[ignore = "not done with process"]
#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}
