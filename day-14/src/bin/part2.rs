use std::{
    collections::HashMap,
    ops::{AddAssign, Neg, SubAssign},
};

const N_CYCLES: u32 = 1_000_000_000;

fn process(mut input: RockGrid) -> String {
    for _ in 0..N_CYCLES {
        input.tilt(Direction::North);
        input.tilt(Direction::West);
        input.tilt(Direction::South);
        input.tilt(Direction::East);
    }

    input.total_load().to_string()
}

struct RockGrid {
    grid: HashMap<Position, Rock>,
    width: usize,
    height: usize,
}

impl RockGrid {
    fn round_rocks(&self) -> impl Iterator<Item = &'_ Position> {
        self.grid
            .iter()
            .filter(|&(_, v)| (*v == Rock::Round))
            .map(|(k, _)| k)
    }

    fn tilt(&mut self, dir: Direction) {
        let mut round_rocks = self.round_rocks().cloned().collect::<Vec<_>>();
        round_rocks.sort_unstable_by_key(|pos| match dir {
            Direction::North => self.height - pos.y,
            Direction::East => self.width - pos.x,
            Direction::South => pos.y,
            Direction::West => pos.x,
        });

        for rock in round_rocks.into_iter() {
            self.roll(rock, dir);
        }
    }

    fn roll(&mut self, mut pos: Position, dir: Direction) {
        let rock = self.grid.remove(&pos).expect("rock not in grid");
        assert!(rock == Rock::Round);

        while self.grid.get(&pos).is_none()
            && (1..=self.width).contains(&pos.x)
            && (1..=self.height).contains(&pos.y)
        {
            pos += dir;
        }
        pos -= dir; // go back to last empty space

        self.grid.insert(pos, rock);
    }

    fn total_load(&self) -> usize {
        self.round_rocks().map(|pos| pos.y).sum::<usize>()
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

// ^ y
// |
// +-> x
// x in 1..=width
// y in 1..=height
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        }
    }
}

impl SubAssign<Direction> for Position {
    fn sub_assign(&mut self, rhs: Direction) {
        *self += -rhs;
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Rock {
    Square,
    Round,
}

impl TryFrom<char> for Rock {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Rock::Square),
            'O' => Ok(Rock::Round),
            c => Err(anyhow::anyhow!("invalid char {c}")),
        }
    }
}

fn parse(input: &str) -> RockGrid {
    let height = input.lines().count();
    let width = input
        .split_once('\n')
        .expect("input is more than one line")
        .0
        .len();
    let input = input.replace('\n', "");

    let grid = input
        .char_indices()
        .fold(HashMap::new(), |mut grid, (idx, char)| {
            if let Ok(rock) = Rock::try_from(char) {
                let coords = Position {
                    x: idx % width + 1,
                    y: height - idx / width,
                };
                grid.insert(coords, rock);
            }
            grid
        });

    RockGrid {
        grid,
        width,
        height,
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
    let example_input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    parse(example_input);
}

#[test]
fn parse_real_input() {
    let real_input = include_str!("input.txt");
    parse(real_input);
}

#[test]
fn example() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let expected = "64";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}

#[ignore = "not done"]
#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "109385";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}
