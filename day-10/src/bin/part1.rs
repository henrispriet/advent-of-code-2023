use std::collections::HashMap;
use enum_iterator::Sequence;

fn process(input: PipeGrid) -> String {
    let (mut walker1, mut walker2) = init_walkers(&input);
    let mut distance = 1;

    while walker1.position != walker2.position {
        walker1.advance();
        walker2.advance();
        distance += 1;
    }

    distance.to_string()
}

#[derive(Debug)]
struct GridWalker<'a> {
    facing: Direction,
    position: (usize, usize),
    grid: &'a PipeGrid,
}

impl GridWalker<'_> {
    fn advance(&mut self) {
        match self.grid.grid[&self.position] {
            Pipe(from, to) if from == -self.facing => self.move_to(to),
            Pipe(to, from) if from == -self.facing => self.move_to(to),
            _ => unreachable!(),
        }
    }

    fn move_to(&mut self, to: Direction) {
        let (x, y) = self.position;
        self.position = match to {
            Direction::North => (x, y.saturating_sub(1)),
            Direction::East => (x + 1, y),
            Direction::West => (x.saturating_sub(1), y),
            Direction::South => (x, y + 1),
        };
        self.facing = to;
    }
}

fn init_walkers(grid: &PipeGrid) -> (GridWalker, GridWalker) {
    let mut walkers = enum_iterator::all::<Direction>().filter_map(|facing| {
        let mut walker = GridWalker {
            facing,
            position: grid.start,
            grid,
        };

        walker.move_to(walker.facing);

        // check walker is looking into pipe
        let Pipe(from1, from2) = grid.grid.get(&walker.position)?;
        (*from1 == -walker.facing || *from2 == -walker.facing).then_some(walker)
    });

    let walker1 = walkers.next().expect("at least one walker");
    let walker2 = walkers.next().expect("at least one walkers");
    (walker1, walker2)
}

#[derive(Debug, Default)]
struct PipeGrid {
    grid: HashMap<(usize, usize), Pipe>,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Sequence)]
enum Direction {
    North,
    East,
    West,
    South,
}

impl std::ops::Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
        }
    }
}

fn parse(input: &str) -> PipeGrid {
    let width = input
        .split_once('\n')
        .expect("input is more than one line")
        .0
        .len();
    let input = input.replace('\n', "");
    let start = input.find('S').expect("input has exactly one S");

    input.char_indices().fold(
        PipeGrid {
            grid: HashMap::new(),
            start: (start % width, start / width),
        },
        |mut grid, (idx, char)| {
            if let Ok(pipe) = Pipe::try_from(char) {
                let coords = (idx % width, idx / width);
                grid.grid.insert(coords, pipe);
            }
            grid
        },
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

#[test]
fn example2() {
    let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    let expected = "8";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}

#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "6860";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}
