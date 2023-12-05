use nom::{
    bytes::complete::tag,
    character::complete::{newline, space0, space1, u32, alpha1, none_of},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair, terminated, tuple, delimited},
    IResult, combinator::not,
};

#[derive(Debug)]
struct Map {
    layers: Vec<MapLayer>,
}

#[derive(Debug)]
struct MapLayer {
    dest_start: u32,
    source_start: u32,
    range_len: u32,
}

type Seed = u32;

fn parse_map_layer(input: &str) -> IResult<&str, MapLayer> {
    let (remainder, (dest_start, _, source_start, _, range_len)) =
        tuple((u32, space1, u32, space1, u32))(input)?;
    Ok((
        remainder,
        MapLayer {
            dest_start,
            source_start,
            range_len,
        },
    ))
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let header = terminated(separated_list1(tag("-"), alpha1), tag(" map:\n"));
    let (remainder, layers) = preceded(header, separated_list1(newline, parse_map_layer))(input)?;
    Ok((remainder, Map { layers }))
}

fn parse(input: &str) -> (Vec<Seed>, Vec<Map>) {
    let parse_seeds = delimited(tag("seeds: "), separated_list1(space1, u32), newline);
    let parse_maps = separated_list1(pair(newline, newline), parse_map);
    let mut parser = separated_pair(parse_seeds, newline, parse_maps);

    match parser(input) {
        Ok(("", output)) => output,
        Ok(output) => panic!("parsing INCOMPLETE!
{output:#?}"),
        Err(error) => panic!("parser FAILED!
{error:#?}"),
    }
}

fn process(input: &str) -> String {
    let (seeds, maps) = parse(input);

    dbg!(seeds);
    dbg!(maps);

    todo!()
}

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    println!("{output}");
}

#[test]
fn example() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    let expected = "35";
    assert_eq!(expected, process(input));
}

#[ignore = "not done"]
#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "";
    assert_eq!(expected, process(input));
}