use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space1, u64},
    combinator::opt,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::ops::Range;

#[derive(Debug)]
struct Map {
    /// sorted by source_start
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

    fn map_range(&self, mut source_range: Range<u64>) -> impl Iterator<Item = Range<u64>> + '_ {
        let mut ranges_per_layer: Vec<Range<u64>> = Vec::new();

        // devide by layer
        for layer in self.layers.iter() {
            let (before, overlap, after) = take(source_range.clone(), layer.source_range());
            if let Some(before) = before {
                ranges_per_layer.push(before);
            }
            if let Some(overlap) = overlap {
                ranges_per_layer.push(overlap);
            }
            if let Some(after) = after {
                source_range = after;
            } else {
                break;
            }
        }
        // all layers were before source_range
        if ranges_per_layer.is_empty() {
            ranges_per_layer.push(source_range);
        }

        // map the things
        ranges_per_layer.into_iter().map(move |range| {
            let len = range.end - range.start;
            let new_start = self.map(range.start);
            new_start..new_start + len
        })
    }
}

// return (before, overlap, after)
// praying to god for no off by one errors
fn take(
    from: Range<u64>,
    range: Range<u64>,
) -> (Option<Range<u64>>, Option<Range<u64>>, Option<Range<u64>>) {
    if range.end <= from.start {
        // from:     |-|
        // range: |-|
        (None, None, Some(from))
    } else if range.start <= from.start {
        if range.end <= from.end {
            // from:   |-|
            // range: |-|
            (None, Some(from.start..range.end), Some(range.end..from.end))
        } else {
            // from:   |-|
            // range: |---|
            (None, Some(from), None)
        }
    } else if range.start <= from.end {
        if range.end <= from.end {
            // from:  |---|
            // range:  |-|
            (
                Some(from.start..range.start),
                Some(range.clone()),
                Some(range.end..from.end),
            )
        } else {
            // from:  |-|
            // range:  |-|
            (
                Some(from.start..range.start),
                Some(range.start..from.end),
                None,
            )
        }
    } else {
        // from:  |-|
        // range:    |-|
        (Some(from), None, None)
    }
}

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
    let (remainder, mut layers) =
        preceded(header, separated_list1(newline, parse_map_layer))(input)?;

    layers.sort_unstable_by_key(|layer| layer.source_start);
    Ok((remainder, Map { layers }))
}

fn parse_seed_range(input: &str) -> IResult<&str, Range<u64>> {
    let (remainder, (start, len)) =
        separated_pair(u64::<&str, nom::error::Error<&str>>, space1, u64)(input)?;
    Ok((remainder, start..start + len))
}

fn parse(input: &str) -> (Vec<Range<u64>>, Vec<Map>) {
    let parse_seeds = delimited(
        tag("seeds: "),
        separated_list1(space1, parse_seed_range),
        newline,
    );
    let parse_maps = terminated(
        separated_list1(pair(newline, newline), parse_map),
        opt(newline),
    );
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
    let mut source = seeds;

    for map in maps {
        // TODO: dont collect every time
        source = source
            .into_iter()
            .flat_map(|range| map.map_range(range))
            .collect();
    }

    source
        .into_iter()
        .map(|range| range.start) // min will be start of a range
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
    let expected = "46";
    assert_eq!(expected, process(input));
}

#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "108956227";
    assert_eq!(expected, process(input));
}
