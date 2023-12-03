use utils::read_file_lines;

fn main() {
    let input_data = read_file_lines("input.txt")
        .expect("Could not read from input file")
        .collect::<Vec<String>>();
    println!("Part one: {}", part_one(&input_data));
    println!("Part two: {}", part_two(&input_data));
}

fn part_one(data: &[String]) -> u32 {
    data.iter()
        .map(|line| {
            let nums = line
                .chars()
                .filter(char::is_ascii_digit)
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            nums.first().unwrap() * 10 + nums.last().unwrap()
        })
        .sum()
}

fn part_two(data: &[String]) -> u32 {
    const DIGIT_STRS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    data.iter()
        .map(|line| {
            let mut nums: Vec<u32> = vec![];
            let mut cur_word = String::new();
            for c in line.chars() {
                if let Some(digit) = c.to_digit(10) {
                    nums.push(digit);
                    cur_word.clear();
                } else {
                    cur_word.push(c);
                    if let Some(idx) = DIGIT_STRS.iter().position(|word| cur_word.contains(word)) {
                        nums.push(idx as u32 + 1);
                        cur_word.clear();
                        // Needed for overlapping edge cases ie "TWONE", "THREEIGHT", etc
                        cur_word.push(c);
                    }
                }
            }
            nums.first().unwrap() * 10 + nums.last().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use utils::read_to_lines;

    const EXAMPLE_ONE_STR: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE_TWO_STR: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn verify_part_one() {
        let data = read_to_lines(EXAMPLE_ONE_STR.as_bytes()).collect::<Vec<String>>();

        let expected = 142;
        let actual = part_one(&data);

        assert_eq!(expected, actual);
    }

    #[test]
    fn verify_part_two() {
        let data = read_to_lines(EXAMPLE_TWO_STR.as_bytes()).collect::<Vec<String>>();

        let expected = 281;
        let actual = part_two(&data);

        assert_eq!(expected, actual);
    }
}
