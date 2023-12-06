use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space1, u64},
    combinator::opt,
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated, tuple},
    Parser,
};
use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
struct Map {
    /// sorted by source_start
    layers: Vec<MapLayer>,
}

#[derive(Debug, PartialEq, Eq)]
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
            let take = take(source_range.clone(), layer.source_range());
            if let Some(before) = take.before {
                ranges_per_layer.push(before);
            }
            if let Some(overlap) = take.overlap {
                ranges_per_layer.push(overlap);
            }
            if let Some(after) = take.after {
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

struct TakeRange {
    before: Option<Range<u64>>,
    overlap: Option<Range<u64>>,
    after: Option<Range<u64>>,
}

fn take(from: Range<u64>, range: Range<u64>) -> TakeRange {
    if range.end <= from.start {
        // from:     |-|
        // range: |-|
        TakeRange {
            before: None,
            overlap: None,
            after: Some(from),
        }
    } else if range.start <= from.start {
        if range.end <= from.end {
            // from:   |-|
            // range: |-|
            TakeRange {
                before: None,
                overlap: Some(from.start..range.end),
                after: Some(range.end..from.end),
            }
        } else {
            // from:   |-|
            // range: |---|
            TakeRange {
                before: None,
                overlap: Some(from),
                after: None,
            }
        }
    } else if range.start <= from.end {
        if range.end <= from.end {
            // from:  |---|
            // range:  |-|
            TakeRange {
                before: Some(from.start..range.start),
                overlap: Some(range.clone()),
                after: Some(range.end..from.end),
            }
        } else {
            // from:  |-|
            // range:  |-|
            TakeRange {
                before: Some(from.start..range.start),
                overlap: Some(range.start..from.end),
                after: None,
            }
        }
    } else {
        // from:  |-|
        // range:    |-|
        TakeRange {
            before: Some(from),
            overlap: None,
            after: None,
        }
    }
}

fn parse(input: &str) -> (Vec<Range<u64>>, Vec<Map>) {
    let seed_ranges = delimited(
        tag("seeds: "),
        separated_list1(
            space1,
            separated_pair(u64, space1, u64).map(|(start, len)| start..start + len),
        ),
        newline,
    );
    let map_layer = tuple((u64, space1, u64, space1, u64)).map(
        |(dest_start, _, source_start, _, range_len)| MapLayer {
            dest_start,
            source_start,
            range_len,
        },
    );
    let map_header = terminated(separated_list1(tag("-"), alpha1), tag(" map:\n"));
    let maps = separated_list1(
        newline,
        delimited(
            map_header,
            separated_list1(newline, map_layer).map(|mut layers| {
                layers.sort_unstable_by_key(|layer| layer.source_start);
                Map { layers }
            }),
            // NOTE: should tecnically be something like newline.or(eof)
            // but this gives a lot of weird errors
            opt(newline),
        ),
    );
    let mut parser =
        // NOTE: should probably have a more descriptive error than ()
        // but this gives a lot of weird errors
        separated_pair::<_, _, _, _, (), _, _, _>(
            seed_ranges, newline, maps,
        );

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

    maps.into_iter()
        .fold(seeds, |source_ranges, map| {
            source_ranges
                .into_iter()
                .flat_map(|range| map.map_range(range))
                // TODO: still the collect here ;-;
                .collect()
        })
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
