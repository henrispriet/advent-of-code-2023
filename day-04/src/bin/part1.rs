// DARN YOU SPACES!
// i hate this function so much
fn despacify(with_spaces: &str) -> String {
    // TODO: SO MANY ALLOCATIONS ;-;
    let despacified = (with_spaces.to_owned() + " ")
        .replace(" 0 ", "00 ")
        .replace(" 1 ", "01 ")
        .replace(" 2 ", "02 ")
        .replace(" 3 ", "03 ")
        .replace(" 4 ", "04 ")
        .replace(" 5 ", "05 ")
        .replace(" 6 ", "06 ")
        .replace(" 7 ", "07 ")
        .replace(" 8 ", "08 ")
        .replace(" 9 ", "09 ");
    despacified[..despacified.len() - 1].to_owned()
}

fn count_intersecting(mut v1: Vec<u32>, mut v2: Vec<u32>) -> usize {
    v1.sort_unstable();
    v2.sort_unstable();

    let mut v1 = v1.into_iter();
    let mut v2 = v2.into_iter();
    let mut count = 0;

    'outer: loop {
        let Some(mut n1) = v1.next() else {
            break;
        };
        let Some(mut n2) = v2.next() else {
            break;
        };

        while n1 != n2 {
            if n1 < n2 {
                n1 = match v1.next() {
                    Some(n) => n,
                    None => break 'outer,
                };
            } else {
                n2 = match v2.next() {
                    Some(n) => n,
                    None => break 'outer,
                }
            }
        }
        count += 1;
    }

    count
}

fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (_, card) = line.split_once(": ").expect("format is 'Card x: '");
            let (winning_numbers_str, my_numbers_str) =
                card.split_once(" | ").expect("format is 'winning | my'");

            let winning_numbers = despacify(winning_numbers_str)
                .split(' ')
                .map(|n| n.parse().expect("is number"))
                .collect();
            let my_numbers = despacify(my_numbers_str)
                .split(' ')
                .map(|n| n.parse().expect("is number"))
                .collect();

            let n_winning_numbers = count_intersecting(winning_numbers, my_numbers) as u32;

            if n_winning_numbers > 0 {
                2u32.pow(n_winning_numbers - 1)
            } else {
                0
            }
        })
        .sum::<u32>()
        .to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    println!("{output}");
}

#[test]
fn example() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let expected = "13";
    assert_eq!(expected, process(input));
}

#[test]
fn real_input() {
    let input = include_str!("input.txt");
    let expected = "21821";
    assert_eq!(expected, process(input));
}
