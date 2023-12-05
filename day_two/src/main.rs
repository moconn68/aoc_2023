mod parser;

use parser::{parse_line, Record};
use utils::read_file_lines;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

fn main() {
    let input: Vec<Record> = read_file_lines("input.txt")
        .expect("Unable to read input file")
        .map(|line| parse_line(&line))
        .collect();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) enum Cube {
    Red,
    Green,
    Blue,
}

impl std::str::FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => Err(())?,
        })
    }
}

fn part_one(data: &[Record]) -> u32 {
    let maxes: HashMap<Cube, u32> =
        HashMap::from([(Cube::Red, 12), (Cube::Green, 13), (Cube::Blue, 14)]);

    data.iter()
        .map(|record| {
            let mut valid = true;
            for game in &record.contents {
                for pull in game {
                    if &pull.1 > maxes.get(&pull.0).unwrap() {
                        valid = false;
                    }
                }
            }
            match valid {
                true => record.id,
                false => 0,
            }
        })
        .sum()
}

fn part_two(data: &[Record]) -> u32 {
    data.iter()
        .map(|record| {
            let mut maxes_map: HashMap<Cube, u32> =
                HashMap::from([(Cube::Red, 0), (Cube::Green, 0), (Cube::Blue, 0)]);
            for pull in record.contents.iter().flatten() {
                let cur_max = maxes_map.get(&pull.0).unwrap();
                if &pull.1 > cur_max {
                    maxes_map.insert(pull.0, pull.1);
                }
            }
            maxes_map.iter().map(|x| x.1).product::<u32>()
        })
        .sum()
}

#[cfg(test)]
pub(crate) mod main_tests {
    use super::*;

    use utils::read_to_lines;

    pub(crate) const EXAMPLE_STR: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn verify_part_one() {
        let input = read_to_lines(EXAMPLE_STR.as_bytes());
        let data: Vec<Record> = input.map(|line| parse_line(&line)).collect();

        let expected = 8;
        let actual = part_one(&data);

        assert_eq!(expected, actual);
    }

    #[test]
    fn verify_part_two() {
        let input = read_to_lines(EXAMPLE_STR.as_bytes());
        let data: Vec<Record> = input.map(|line| parse_line(&line)).collect();

        let expected = 2286;
        let actual = part_two(&data);

        assert_eq!(expected, actual);
    }
}
