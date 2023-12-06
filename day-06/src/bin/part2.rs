fn process(input: BoatRaces) -> String {
    input
        .into_iter()
        .map(|(time, distance_to_beat)| {
            // num ways to win
            // speed = time_held
            // dist = speed * (time - time_held)
            //
            // so with x = time_held, t = time, d = dist:
            // -x^2 + t*x - d = 0
            // solved, this means:
            // D = t^2 - 4*d
            let discriminant = (time * time - 4 * distance_to_beat) as f32;

            // x1 = (-t + sqrt(D)) / -2
            let x1 = (-(time as f32) + discriminant.sqrt()) / -2.;
            // x2 = (-t - sqrt(D)) / -2
            let x2 = (-(time as f32) - discriminant.sqrt()) / -2.;
            // with x1 <= x2
            debug_assert!(x1 <= x2);

            // because our distance has to be _strictly_ greater (>)
            // than distance_to_beat
            let lower_bound = (x1 + 1.).floor() as u64; // ~x1.ceil()
            let upper_bound = (x2 - 1.).ceil() as u64; // ~x2.floor()

            // num_ways_to_win =
            (lower_bound..=upper_bound).count()
        })
        .product::<usize>()
        .to_string()
}

#[derive(Debug)]
struct BoatRaces {
    times: Vec<u64>,
    distances: Vec<u64>,
}

impl IntoIterator for BoatRaces {
    type Item = (u64, u64);

    type IntoIter = std::iter::Zip<std::vec::IntoIter<u64>, std::vec::IntoIter<u64>>;

    fn into_iter(self) -> Self::IntoIter {
        self.times.into_iter().zip(self.distances)
    }
}

fn parse(input: &str) -> BoatRaces {
    let no_spaces = input.replace(' ', "");
    let mut lines = no_spaces.lines();
    let mut get_the_fricking_number = || {
        lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .parse::<u64>()
            .unwrap()
    };

    let time = get_the_fricking_number();
    let distance = get_the_fricking_number();

    BoatRaces {
        times: vec![time],
        distances: vec![distance],
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
    let example_input = "Time:      7  15   30
Distance:  9  40  200";
    parse(example_input);
}

#[test]
fn parse_real_input() {
    let real_input = include_str!("input.txt");
    parse(real_input);
}

#[test]
fn example() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    let expected = "71503";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}

#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "32607562";

    let parsed = parse(input);
    let output = process(parsed);
    assert_eq!(expected, output);
}
