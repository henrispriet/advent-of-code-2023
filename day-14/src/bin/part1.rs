use std::collections::HashMap;

fn process(mut input: RockGrid) -> String {
    let mut round_rocks = input.round_rocks().cloned().collect::<Vec<_>>();
    round_rocks.sort_unstable_by_key(|pos| pos.y);

    for rock in round_rocks.into_iter().rev() {
        input.roll_north(rock);
    }
    input.total_load().to_string()
}

struct RockGrid {
    grid: HashMap<Position, Rock>,
    height: usize,
}

impl RockGrid {
    fn round_rocks(&self) -> impl Iterator<Item = &'_ Position> {
        self.grid
            .iter()
            .filter(|&(_, v)| (*v == Rock::Round))
            .map(|(k, _)| k)
    }

    fn roll_north(&mut self, mut pos: Position) {
        let rock = self.grid.remove(&pos).expect("rock not in grid");
        assert!(rock == Rock::Round);

        while self.grid.get(&pos).is_none() && pos.y <= self.height {
            pos.y += 1;
        }
        pos.y -= 1; // go back to last empty space

        self.grid.insert(pos, rock);
    }

    fn total_load(&self) -> usize {
        self.round_rocks().map(|pos| pos.y).sum::<usize>()
    }
}

// ^ y
// |
// +-> x
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
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
                    x: idx % width,
                    y: height - idx / width,
                };
                grid.insert(coords, rock);
            }
            grid
        });

    RockGrid { grid, height }
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
    let expected = "136";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}

#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "109385";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}
