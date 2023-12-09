use nom::{
    character::complete::{i32, line_ending, space1},
    combinator::opt,
    multi::separated_list1,
    Parser,
};
use nom_supreme::{error::ErrorTree, ParserExt};

fn process(input: ParsedData) -> String {
    input
        .into_iter()
        .map(|seq| seq.next_value())
        .sum::<i32>()
        .to_string()
}

type ParsedData = Vec<Sequence>;

#[derive(Debug)]
struct Sequence(Vec<i32>);

impl Sequence {
    fn next_value(&self) -> i32 {
        if self.0.iter().all(|e| *e == 0) {
            0
        } else {
            self.0.last().expect("sequence contains at least 1 element")
                + self.derivative().next_value()
        }
    }

    fn derivative(&self) -> Sequence {
        let mut deltas = Vec::new();
        for i in 0..self.0.len() - 1 {
            deltas.push(self.0[i + 1] - self.0[i]);
        }
        Sequence(deltas)
    }
}

fn parse(input: &str) -> ParsedData {
    let sequence = separated_list1(space1::<_, ErrorTree<&str>>, i32).map(Sequence);
    let mut parser = separated_list1(line_ending, sequence).terminated(opt(line_ending));

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
    let example_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    parse(example_input);
}

#[test]
fn parse_real_input() {
    let real_input = include_str!("input.txt");
    parse(real_input);
}

#[test]
fn example() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    let expected = "114";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}

#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "1938800261";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}

