use std::collections::HashSet;

use itertools::Itertools;

fn process(input: GalaxyGrid) -> String {
    // because `cartesian_product` takes each pair twice (and im too lazy to fix it)
    // TODO: fix it
    let double_output = input
        .grid
        .iter()
        .cartesian_product(input.grid.iter())
        .map(|(&p1, &p2)| input.distance(p1, p2))
        .sum::<usize>();

    (double_output / 2).to_string()
}

#[derive(Debug)]
struct GalaxyGrid {
    grid: HashSet<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
}

impl GalaxyGrid {
    fn distance(&self, p1: (usize, usize), p2: (usize, usize)) -> usize {
        let (p1, p2) = (
            (p1.0.min(p2.0), p1.1.min(p2.1)),
            (p1.0.max(p2.0), p1.1.max(p2.1)),
        );

        let base_distance = (p2.1 - p1.1) + (p2.0 - p1.0);

        let mut expanded_distance = base_distance;
        for row in p1.1..p2.1 {
            if self.empty_rows.contains(&row) {
                expanded_distance += 1;
            }
        }
        for column in p1.0..p2.0 {
            if self.empty_columns.contains(&column) {
                expanded_distance += 1;
            }
        }

        expanded_distance
    }
}

fn inverse<'a>(original: impl Iterator<Item = &'a usize>) -> Vec<usize> {
    let mut inverse = Vec::new();
    let mut prev = 0;
    for &value in original.sorted() {
        inverse.extend_from_slice(&(prev + 1..value).collect::<Vec<_>>());
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
    let expected = "374";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}

#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "10313550";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}
