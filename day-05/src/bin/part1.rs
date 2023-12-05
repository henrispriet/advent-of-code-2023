use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space1, u64},
    combinator::opt,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug)]
struct Map {
    layers: Vec<MapLayer>,
}

#[derive(Debug)]
struct MapLayer {
    dest_start: u64,
    source_start: u64,
    range_len: u64,
}

impl MapLayer {
    fn source_range(&self) -> Range<u64> {
        self.source_start..self.source_start + self.range_len
    }
}

impl Map {
    fn map(&self, source: u64) -> u64 {
        self.layers
            .iter()
            .find_map(|layer| {
                layer.source_range().contains(&source).then(|| {
                    let offset = source - layer.source_start;
                    layer.dest_start + offset
                })
            })
            .unwrap_or(source)
    }
}

type Seed = u64;

fn parse_map_layer(input: &str) -> IResult<&str, MapLayer> {
    let (remainder, (dest_start, _, source_start, _, range_len)) =
        tuple((u64, space1, u64, space1, u64))(input)?;
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
    let parse_seeds = delimited(tag("seeds: "), separated_list1(space1, u64), newline);
    let parse_maps = terminated(separated_list1(pair(newline, newline), parse_map), opt(newline));
    let mut parser = separated_pair(parse_seeds, newline, parse_maps);

    match parser(input) {
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

fn process(input: &str) -> String {
    let (seeds, maps) = parse(input);

    seeds
        .into_iter()
        .map(|seed| {
            let mut source = seed;
            for map in maps.iter() {
                source = map.map(source);
            }
            source
        })
        .min()
        .expect("at least one seed")
        .to_string()
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

#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "322500873";
    assert_eq!(expected, process(input));
}
