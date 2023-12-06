use nom::{
    character::complete::{line_ending, space0, space1, u32},
    combinator::opt,
    multi::separated_list1,
    sequence::separated_pair,
    Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

fn process(input: BoatRaces) -> String {
    input
        .into_iter()
        .map(|(time, distance_to_beat)| {
            // num ways to win
            // speed = time_held
            // dist = speed * (time - time_held)
            //
            // so with x = time_held, t = time, d = dist:
            // -x^2 + t*x - d = 0
            // solved, this means:
            // D = t^2 - 4*d
            let discriminant = (time * time - 4 * distance_to_beat) as f32;

            // x1 = (-t + sqrt(D)) / -2
            let x1 = (-(time as f32) + discriminant.sqrt()) / -2.;
            // x2 = (-t - sqrt(D)) / -2
            let x2 = (-(time as f32) - discriminant.sqrt()) / -2.;
            // with x1 <= x2
            debug_assert!(x1 <= x2);

            // because our distance has to be _strictly_ greater (>)
            // than distance_to_beat
            let lower_bound = (x1 + 1.).floor() as u32; // ~x1.ceil()
            let upper_bound = (x2 - 1.).ceil() as u32; // ~x2.floor()

            // num_ways_to_win =
            (lower_bound..=upper_bound).count()
        })
        .product::<usize>()
        .to_string()
}

#[derive(Debug)]
struct BoatRaces {
    times: Vec<u32>,
    distances: Vec<u32>,
}

impl IntoIterator for BoatRaces {
    type Item = (u32, u32);

    type IntoIter = std::iter::Zip<std::vec::IntoIter<u32>, std::vec::IntoIter<u32>>;

    fn into_iter(self) -> Self::IntoIter {
        self.times.into_iter().zip(self.distances)
    }
}

fn parse(input: &str) -> BoatRaces {
    let list = |name: &'static str| {
        tag(name)
            .precedes(space0)
            .precedes(separated_list1(space1::<&str, ErrorTree<&str>>, u32))
    };
    let mut parser = separated_pair(list("Time:"), line_ending, list("Distance:"))
        .terminated(opt(line_ending))
        .map(|(times, distances)| BoatRaces { times, distances });

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

#[test]
fn example() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    let expected = "288";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}

#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "503424";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}
