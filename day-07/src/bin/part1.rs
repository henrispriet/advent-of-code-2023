use std::str::FromStr;

use nom::{
    character::complete::{alphanumeric1, line_ending, space1, u32},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated}, Parser,
};
use nom_supreme::{error::ErrorTree, ParserExt};

fn process(input: Game) -> String {
    todo!()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FiveOfAKind,
}

#[derive(Debug, PartialEq)]
struct Hand([Card; 5]);

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand(
            s.chars()
                .map(Card::try_from)
                .collect::<Result<Vec<_>, _>>()?
                .try_into()
                .map_err(|e: Vec<_>| anyhow::anyhow!("unexpected hand length: {}", e.len()))?,
        ))
    }
}

type Game = Vec<(Hand, u32)>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    value: u8,
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let value = match (value.to_digit(10), value) {
            (Some(n), _) if (2..=9).contains(&n) => n as u8,
            (None, 'T') => 10,
            (None, 'J') => 11,
            (None, 'Q') => 12,
            (None, 'K') => 13,
            (None, 'A') => 14,
            _ => anyhow::bail!("invalid card {value}"),
        };

        Ok(Card { value })
    }
}

fn parse(input: &str) -> Game {
    let line = separated_pair(
        alphanumeric1::<_, ErrorTree<&str>>.map_res(Hand::from_str),
        space1,
        u32,
    );
    let mut parser = separated_list1(
        line_ending,
        line,
    ).terminated(line_ending.opt());

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
    let example_input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
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
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    let expected = "6440";

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
