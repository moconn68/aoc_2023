/// Cardinal directions for adjacency checking in a 2D array, including diagonal.
const ADJ_DIRS: [(i16, i16); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

/// Represents a coordinate in a 2D array.
/// _Slight_ clarity improvement over `(usize, usize)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Self {
            row: value.0,
            col: value.1,
        }
    }
}

/// A [`u8`] wrapper with validity checking for an ASCII numeric digit \[0-9\].
pub struct Digit(u8);

impl TryFrom<u8> for Digit {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 9 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

impl TryFrom<char> for Digit {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let byte_u32 = value.to_digit(10).ok_or(())?;
        let byte = u8::try_from(byte_u32).map_err(|_| ())?;
        Self::try_from(byte)
    }
}

impl std::ops::Deref for Digit {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Converts a list of [`Digit`]s to a number ([`u32`]).
///
/// ## Example
/// ```
/// use day_three::{Digit, digits_to_number};
///
/// let digits_u8: [u8; 3] = [1, 2, 3];
/// let digits: Vec<Digit> = digits_u8
///     .into_iter()
///     .filter_map(|x| Some(Digit::try_from(x).unwrap()))
///     .collect();
/// assert_eq!(123, digits_to_number(&digits));
/// ```
pub fn digits_to_number(digits: &[Digit]) -> u32 {
    digits
        .iter()
        .fold(0, |acc, digit| acc * 10 + (**digit as u32))
}

/// For all spaces in a 2-D matrix that are adjacent to a given position, get their coordinates and
/// the element data. This also accounts for array boundaries and will only return valid data.
///
/// * `coord`: The coordinate to look for adjacencies
/// * `data`: 2D data array
///
/// Returns a list of valid adjacent elements. This data is represented by a tuple,
/// where the first element is the [`Coord`] position and the second is a reference to
/// the element itself.
///
/// ## Example
/// ```
/// use day_three::{Coord, get_adjacent_data};
/// let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
///
/// let expected: Vec<(Coord, &i32)> = vec![
///     (Coord::new(0, 1), &2),
///     (Coord::new(1, 0), &4),
///     (Coord::new(1, 1), &5),
/// ];
/// let actual = get_adjacent_data((0, 0).into(), &data);
/// assert_eq!(expected, actual);
/// ```
pub fn get_adjacent_data<T>(coord: Coord, data: &[Vec<T>]) -> Vec<(Coord, &T)> {
    ADJ_DIRS
        .into_iter()
        .filter_map(|(dy, dx)| {
            let adj_pos = Coord::new(
                (coord.row as i16 + dy) as usize,
                (coord.col as i16 + dx) as usize,
            );
            data.get(adj_pos.row)?
                .get(adj_pos.col)
                .map(|el| (adj_pos, el))
        })
        .collect()
}

/// Prints the input 2D array (grid) to stdout in a human-readible manner.
///
/// ## Example
/// ```
/// use day_three::pretty_print_grid;
///
/// const EXAMPLE_STR: &str =
/// r"467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..";
///
/// let example_grid = utils::read_to_lines(EXAMPLE_STR.as_bytes())
///     .map(|line| line.chars().collect())
///     .collect::<Vec<Vec<char>>>();
///
/// pretty_print_grid(example_grid.iter());
/// /* Output:
/// ['4', '6', '7', '.', '.', '1', '1', '4', '.', '.']
/// ['.', '.', '.', '*', '.', '.', '.', '.', '.', '.']
/// ['.', '.', '3', '5', '.', '.', '6', '3', '3', '.']
/// ['.', '.', '.', '.', '.', '.', '#', '.', '.', '.']
/// ['6', '1', '7', '*', '.', '.', '.', '.', '.', '.']
/// ['.', '.', '.', '.', '.', '+', '.', '5', '8', '.']
/// ['.', '.', '5', '9', '2', '.', '.', '.', '.', '.']
/// ['.', '.', '.', '.', '.', '.', '7', '5', '5', '.']
/// ['.', '.', '.', '$', '.', '*', '.', '.', '.', '.']
/// ['.', '6', '6', '4', '.', '5', '9', '8', '.', '.']
/// */
/// ```
pub fn pretty_print_grid<T: std::fmt::Debug>(grid: impl Iterator<Item = T>) {
    grid.for_each(|row| println!("{row:?}"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacent_corner() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let expected: Vec<(Coord, &i32)> = vec![
            (Coord::new(0, 1), &2),
            (Coord::new(1, 0), &4),
            (Coord::new(1, 1), &5),
        ];
        let actual = get_adjacent_data((0, 0).into(), &data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn adjacent_side() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let expected: Vec<(Coord, &i32)> = vec![
            (Coord::new(0, 0), &1),
            (Coord::new(0, 2), &3),
            (Coord::new(1, 0), &4),
            (Coord::new(1, 1), &5),
            (Coord::new(1, 2), &6),
        ];
        let actual = get_adjacent_data((0, 1).into(), &data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn adjacent_middle() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let expected: Vec<(Coord, &i32)> = vec![
            (Coord::new(0, 0), &1),
            (Coord::new(0, 1), &2),
            (Coord::new(0, 2), &3),
            (Coord::new(1, 0), &4),
            (Coord::new(1, 2), &6),
            (Coord::new(2, 0), &7),
            (Coord::new(2, 1), &8),
            (Coord::new(2, 2), &9),
        ];
        let actual = get_adjacent_data((1, 1).into(), &data);
        assert_eq!(expected, actual);
    }
}
