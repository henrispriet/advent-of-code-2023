use nom::{
    character::complete::{alphanumeric1, anychar, line_ending},
    combinator::opt,
    multi::{fold_many1, many1},
    sequence::{delimited, separated_pair},
    Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};
use std::{collections::HashMap, str::FromStr};

const START_CHAR: char = 'A';
const END_CHAR: char = 'Z';

fn process(input: ParsedData) -> String {
    let ParsedData { instructions, map } = input;
    let current_nodes = map
        .keys()
        .filter(|node| node.is_start())
        .collect::<Vec<_>>();

    // calculate path lengths separately
    let num_steps = current_nodes.into_iter().map(|node| {
        let mut num_steps = 0;
        let mut current_node = node;

        'outer: loop {
            for instruction in instructions.iter() {
                current_node = map.get(current_node).expect("node in map").go(instruction);
                num_steps += 1;

                if current_node.is_end() {
                    break 'outer;
                }
            }
        }

        num_steps
    });

    // total is the lowest common multiple (lcm)
    // lcm of n numbers lcm(a[..n]) = lcm(lcm(a[..n-1], a[n-1]))
    num_steps
        .fold(1u64, num::integer::lcm)
        .to_string()
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            c => Err(anyhow::anyhow!("invalid char: {c}")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node([char; 3]);

impl Node {
    fn is_start(&self) -> bool {
        self.0[2] == START_CHAR
    }

    fn is_end(&self) -> bool {
        self.0[2] == END_CHAR
    }
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        anyhow::ensure!(s.chars().count() == 3, "wrong len: {}", s.len());

        let mut out = ['\0'; 3];
        let mut chars = s.chars();
        out[0] = chars.next().expect("len 3");
        out[1] = chars.next().expect("len 3");
        out[2] = chars.next().expect("len 3");

        Ok(Node(out))
    }
}

#[derive(Debug)]
struct Junction {
    left: Node,
    right: Node,
}

impl Junction {
    #[inline]
    fn go(&self, direction: &Direction) -> &Node {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

#[derive(Debug)]
struct ParsedData {
    instructions: Vec<Direction>,
    map: HashMap<Node, Junction>,
}

fn parse(input: &str) -> ParsedData {
    let instructions =
        many1(anychar::<_, ErrorTree<&str>>.map_res(Direction::try_from)).terminated(line_ending);
    let junction = separated_pair(
        alphanumeric1.map_res(Node::from_str),
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(
                alphanumeric1.map_res(Node::from_str),
                tag(", "),
                alphanumeric1.map_res(Node::from_str),
            ),
            tag(")"),
        ),
    );
    let parse_map = fold_many1(
        junction.terminated(opt(line_ending)),
        HashMap::new,
        |mut acc, (from, (left, right))| {
            acc.insert(from, Junction { left, right });
            acc
        },
    );
    let mut parser = separated_pair(instructions, line_ending, parse_map)
        .map(|(instructions, map)| ParsedData { instructions, map });

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
fn parse_example1() {
    let example_input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    parse(example_input);
}

#[test]
fn parse_example2() {
    let example_input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    parse(example_input);
}

#[test]
fn parse_example3() {
    let example_input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    parse(example_input);
}

#[test]
fn parse_real_input() {
    let real_input = include_str!("input.txt");
    parse(real_input);
}

#[test]
fn example1() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    let expected = "2";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}

#[test]
fn example2() {
    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    let expected = "6";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}

#[test]
fn example3() {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    let expected = "6";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}

#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "13385272668829";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}
