use day_three::{digits_to_number, get_adjacent_data, Coord, Digit};
use utils::read_file_lines;

use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_file_lines("input.txt").expect("Could not read from file");
    let data: Vec<Vec<char>> = input
        .map(|mut line| {
            // Pad each line with empty space as easy workaround to ensure part numbers at the end of a row are counted.
            line.push('.');
            line.chars().collect::<Vec<char>>()
        })
        .collect();

    println!("Part one: {}", part_one(&data));
    println!("Part two: {}", part_two(&data));
}

fn part_one(data: &[Vec<char>]) -> u32 {
    let mut sum = Default::default();
    for (ridx, row) in data.iter().enumerate() {
        let mut digits: Vec<Digit> = vec![];
        let mut has_symbol = false;
        for (cidx, el) in row.iter().enumerate() {
            let coord = Coord::new(ridx, cidx);
            if let Ok(digit) = Digit::try_from(*el) {
                /*
                 * Case - digit
                 * Push digit onto `digits` stack and do symbol adjacency check.
                 */
                digits.push(digit);
                let adj = get_adjacent_data(coord, data);
                if !has_symbol
                    && adj
                        .iter()
                        .filter(|(_, c)| '.'.ne(c) && !c.is_ascii_digit())
                        .count()
                        > 0
                {
                    has_symbol = true;
                }
            } else {
                /*
                 * Case - empty | symbol
                 * If `digits` is empty then we proceed as normal.
                 * Otherwise, if `has_symbol` is true then we convert `digits` to a number.
                 * Finally, we clear `digits` and reset `has_symbol`.
                 */
                if !digits.is_empty() && has_symbol {
                    let num = digits_to_number(&digits);
                    sum += num;
                }
                digits.clear();
                has_symbol = false;
            }
        }
    }
    sum
}

fn part_two(data: &[Vec<char>]) -> u32 {
    let mut gear_map: HashMap<Coord, Vec<u32>> = Default::default();
    for (ridx, row) in data.iter().enumerate() {
        let mut digits: Vec<Digit> = vec![];
        let mut adj_gears: HashSet<Coord> = Default::default();
        for (cidx, el) in row.iter().enumerate() {
            let coord = Coord::new(ridx, cidx);
            if let Ok(digit) = Digit::try_from(*el) {
                /*
                 * Case - digit
                 * Push digit onto `digits` stack.
                 * If there is a gear ('*') adjacent to the digit, add its coordinates to the set.
                 */
                digits.push(digit);
                let adj_data = get_adjacent_data(coord, data);
                for (pos, el) in adj_data {
                    if '*'.eq(el) {
                        adj_gears.insert(pos);
                    }
                }
            } else {
                /*
                 * Case - empty | symbol
                 * If `digits` or `adj_gears` is empty then we proceed as normal.
                 * Otherwise, iterate through the coordinates of the found gears and add the newly created number
                 * to the list of numbers which are adjacent to said gear.
                 * Finally, we clear `digits` and `adj_gears`.
                 */
                if !digits.is_empty() && !adj_gears.is_empty() {
                    let num = digits_to_number(&digits);
                    for gear in &adj_gears {
                        match gear_map.get_mut(gear) {
                            Some(pos) => pos.push(num),
                            None => _ = gear_map.insert(*gear, vec![num]),
                        }
                    }
                }
                digits.clear();
                adj_gears.clear();
            }
        }
    }
    // Disregard any gears which have a quantity of adjacent numbers != 2.
    // For those that remain, get the gear ratio by multiplying the value of the two adjacent numbers.
    // Return the sum of all of these ratios.
    gear_map
        .into_iter()
        .filter_map(|(_, nums)| {
            if nums.len() == 2 {
                Some(nums[0] * nums[1])
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use utils::read_to_lines;

    const EXAMPLE_STR: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn verify_part_one() {
        let input = read_to_lines(EXAMPLE_STR.as_bytes())
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let expected = 4361;
        let actual = part_one(&input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn verify_part_two() {
        let input = read_to_lines(EXAMPLE_STR.as_bytes())
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let expected = 467835;
        let actual = part_two(&input);

        assert_eq!(expected, actual);
    }
}
