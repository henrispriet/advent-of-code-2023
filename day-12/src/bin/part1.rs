use itertools::Itertools;
use nom::{
    bytes::complete::take_until,
    character::complete::{char, line_ending, u32},
    combinator::opt,
    multi::separated_list1,
    sequence::separated_pair,
    Parser,
};
use nom_supreme::{error::ErrorTree, ParserExt};

fn process(input: ParsedData) -> String {
    input
        .into_iter()
        .map(|mut row| {
            let n_missing_broken = row.broken_counts.iter().sum::<usize>()
                - row
                    .row
                    .iter()
                    .filter(|&&s| s == SpringStatus::Broken)
                    .count();

            // brute force
            // ripped from https://stackoverflow.com/questions/44139493/in-rust-what-is-the-proper-way-to-replicate-pythons-repeat-parameter-in-iter
            let all_combinations =
                std::iter::repeat([SpringStatus::Working, SpringStatus::Broken].into_iter())
                    .take(row.unknown_idx.len())
                    .multi_cartesian_product();

            all_combinations
                .filter(|combination| {
                    if combination
                        .iter()
                        .filter(|&&s| s == SpringStatus::Broken)
                        .count()
                        != n_missing_broken
                    {
                        return false;
                    }

                    for (&idx, &status) in row.unknown_idx.iter().zip(combination) {
                        row.row[idx] = status;
                    }
                    row.is_valid()
                })
                .count()
        })
        .sum::<usize>()
        .to_string()
}

type ParsedData = Vec<SpringRow>;

#[derive(Debug)]
struct SpringRow {
    row: Vec<SpringStatus>,
    unknown_idx: Vec<usize>,
    broken_counts: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringStatus {
    Working,
    Broken,
    Unknown,
}

impl TryFrom<char> for SpringStatus {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(SpringStatus::Working),
            '#' => Ok(SpringStatus::Broken),
            '?' => Ok(SpringStatus::Unknown),
            _ => Err(anyhow::anyhow!("invalid char")),
        }
    }
}

impl SpringRow {
    fn is_valid(&self) -> bool {
        let mut counts = Vec::new();
        let mut new_group = true;

        for status in &self.row {
            match status {
                SpringStatus::Working => {
                    new_group = true;
                }
                SpringStatus::Broken => {
                    if new_group {
                        counts.push(1);
                    } else {
                        // new_group starts at true, so at this point at least one count
                        *counts.last_mut().unwrap() += 1;
                    }

                    new_group = false;
                }
                SpringStatus::Unknown => return false,
            }
        }

        counts == self.broken_counts
    }
}

fn parse(input: &str) -> ParsedData {
    let counts = separated_list1(char(','), u32.map(|n| n as usize));
    let row = separated_pair(take_until(" "), char(' '), counts).map(
        |(row, broken_counts): (&str, _)| {
            let unknown_idx = row
                .match_indices('?')
                .map(|(idx, _)| idx)
                .collect::<Vec<_>>();
            let row = row
                .chars()
                .map(|c| SpringStatus::try_from(c).unwrap())
                .collect();
            SpringRow {
                row,
                unknown_idx,
                broken_counts,
            }
        },
    );
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

#[test]
fn example() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    let expected = "21";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}

#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "7753";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}
