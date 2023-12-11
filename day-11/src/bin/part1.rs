use std::collections::HashSet;

use itertools::Itertools;

fn process(_input: GalaxyGrid) -> String {
    todo!()
}

#[derive(Debug)]
struct GalaxyGrid {
    grid: HashSet<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
}

fn inverse<'a>(original: impl Iterator<Item = &'a usize>) -> Vec<usize> {
    let mut inverse = Vec::new();
    let mut prev = 0;
    for &value in original {
        inverse.extend_from_slice(&(prev..value).collect::<Vec<_>>());
        prev = value;
    }

    inverse
}

impl From<HashSet<(usize, usize)>> for GalaxyGrid {
    fn from(value: HashSet<(usize, usize)>) -> Self {
        let not_empty_rows = value.iter().map(|(_, y)| y).unique();
        let not_empty_columns = value.iter().map(|(x, _)| x).unique();

        let empty_rows = inverse(not_empty_rows);
        let empty_columns = inverse(not_empty_columns);

        GalaxyGrid {
            grid: value,
            empty_rows,
            empty_columns,
        }
    }
}

fn parse(input: &str) -> GalaxyGrid {
    let width = input
        .split_once('\n')
        .expect("input is more than one line")
        .0
        .len();
    let input = input.replace('\n', "");

    input
        .char_indices()
        .fold(HashSet::new(), |mut grid, (idx, char)| {
            match char {
                '#' => {
                    let coords = (idx % width, idx / width);
                    grid.insert(coords);
                }
                '.' => {}
                _ => unreachable!(),
            }
            grid
        })
        .into()
}

fn main() {
    let input = include_str!("input.txt");

    let parsed = parse(input);
    let output = process(parsed);
    println!("{output}");
}

#[test]
fn parse_example() {
    let example_input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
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
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
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
