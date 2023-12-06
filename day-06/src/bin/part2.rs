use nom::{
    character::complete::{alpha1, digit1, line_ending, space0, space1, u64},
    combinator::opt,
    multi::separated_list1,
    sequence::separated_pair,
    Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

fn process(input: BoatRace) -> String {
    // num ways to win
    // speed = time_held
    // dist = speed * (time - time_held)
    //
    // so with x = time_held, t = time, d = dist:
    // -x^2 + t*x - d = 0
    // solved, this means:
    // D = t^2 - 4*d
    let discriminant = (input.time * input.time - 4 * input.distance) as f32;

    // x1 = (-t + sqrt(D)) / -2
    let x1 = (-(input.time as f32) + discriminant.sqrt()) / -2.;
    // x2 = (-t - sqrt(D)) / -2
    let x2 = (-(input.time as f32) - discriminant.sqrt()) / -2.;
    // with x1 <= x2
    debug_assert!(x1 <= x2);

    // because our distance has to be _strictly_ greater (>)
    // than distance_to_beat
    let lower_bound = (x1 + 1.).floor() as u64; // ~x1.ceil()
    let upper_bound = (x2 - 1.).ceil() as u64; // ~x2.floor()

    // num_ways_to_win =
    (lower_bound..=upper_bound).count().to_string()
}

#[derive(Debug)]
struct BoatRace {
    time: u64,
    distance: u64,
}

fn parse(input: &str) -> BoatRace {
    let number = |name: &'static str| {
        tag(name)
            .precedes(space0::<&str, ErrorTree<&str>>)
            .precedes(separated_list1(space1, digit1))
            .map(|list| {
                list.join("")
                    .parse()
                    // dont like this
                    .expect("time and distance are valid numbers")
            })
    };
    let mut parser = separated_pair(number("Time:"), line_ending, number("Distance:"))
        .terminated(opt(line_ending))
        .map(|(time, distance)| BoatRace { time, distance });

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
    let expected = "71503";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}

#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "32607562";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}
