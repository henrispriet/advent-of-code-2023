use nom::{
    character::complete::{char, line_ending, u32},
    combinator::opt,
    multi::{many0, many1, many1_count, separated_list1},
    sequence::separated_pair,
    Parser,
};
use nom_supreme::{error::ErrorTree, ParserExt};

fn process(_input: ParsedData) -> String {
    todo!()
}

type ParsedData = Vec<SpringRow>;

#[derive(Debug)]
struct SpringRow {
    groups: Vec<SpringGroup>,
    broken_counts: Vec<u32>,
}

#[derive(Debug)]
enum SpringGroup {
    Unknown(u32),
    Broken(u32),
}

fn parse(input: &str) -> ParsedData {
    let groups = many1(
        many0(char('.')).precedes(
            many1_count(char('#'))
                .map(|n| SpringGroup::Broken(n as u32))
                .or(many1_count(char('?')).map(|n| SpringGroup::Unknown(n as u32))),
        ),
    )
    .terminated(many0(char('.')));
    let counts = separated_list1(char(','), u32);
    let row = separated_pair(groups, char(' '), counts).map(|(groups, broken_counts)| SpringRow {
        groups,
        broken_counts,
    });
    let parser = separated_list1(line_ending, row);

    match parser
        .terminated(opt(line_ending::<_, ErrorTree<&str>>))
        .parse(input)
    {
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
    let example_input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
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
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    let expected = "";

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
