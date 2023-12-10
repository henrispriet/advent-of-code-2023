use std::collections::HashMap;

fn process(_input: PipeGrid) -> String {
    todo!()
}

#[derive(Debug, Default)]
struct PipeGrid {
    grid: HashMap<(usize, usize), Pipe>,
    width: usize,
    start: (usize, usize),
}

#[derive(Debug)]
struct Pipe(Direction, Direction);

impl TryFrom<char> for Pipe {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Pipe(Direction::North, Direction::South)),
            '-' => Ok(Pipe(Direction::West, Direction::East)),
            'L' => Ok(Pipe(Direction::North, Direction::East)),
            'J' => Ok(Pipe(Direction::North, Direction::West)),
            '7' => Ok(Pipe(Direction::South, Direction::West)),
            'F' => Ok(Pipe(Direction::South, Direction::East)),
            _ => Err(anyhow::anyhow!("not a pipe char: {value}")),
        }
    }
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn parse(input: &str) -> PipeGrid {
    let width = input
        .split_once('\n')
        .expect("input is more than one line")
        .0
        .len();
    let input = input.replace('\n', "");
    let start = input.find('S').expect("input has exactly one S");

    input.char_indices().fold(PipeGrid{
        grid: HashMap::new(),
        width,
        start: (start / width, start % width),
    },
        |mut grid, (idx, char)| {
            if let Ok(pipe) = Pipe::try_from(char) {
                let coords = (idx / grid.width, idx % grid.width);
                grid.grid.insert(coords, pipe);
            }
            grid
        }
    )
}

fn main() {
    let input = include_str!("input.txt");

    let parsed = parse(input);
    let output = process(parsed);
    println!("{output}");
}

#[test]
fn parse_example() {
    let example_input = ".....
.S-7.
.|.|.
.L-J.
.....";
    parse(example_input);
}

#[test]
fn parse_example2() {
    let example_input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
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
    let input = ".....
.S-7.
.|.|.
.L-J.
.....";
    let expected = "4";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}

#[ignore = "not done with parse"]
#[test]
fn example2() {
    let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    let expected = "8";

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
