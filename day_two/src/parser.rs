use crate::Cube;

use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Record {
    pub id: u32,
    // Pair of cube color and maximum number of said color pulled in the game
    pub contents: Vec<Vec<(Cube, u32)>>,
}

pub fn parse_line(line: &str) -> Record {
    let (id_str, contents_str) = line.split_once(':').unwrap();
    let id = id_str.split(' ').last().unwrap().parse::<u32>().unwrap();

    let mut contents: Vec<Vec<(Cube, u32)>> = vec![];
    let pulls = contents_str.split(';');
    for pull in pulls.map(str::trim) {
        let mut pull_list: Vec<(Cube, u32)> = vec![];
        for cubes in pull.split(',').map(str::trim) {
            let (num, color) = cubes.split_once(' ').unwrap();
            let count = num.parse::<u32>().unwrap();
            let cube = Cube::from_str(color).unwrap();

            pull_list.push((cube, count));
        }
        contents.push(pull_list);
    }

    Record { id, contents }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::main_tests::EXAMPLE_STR;

    const EXAMPLE_RECORD_JSON: &str = r#"[{"id":1,"contents":[[["Blue",3],["Red",4]],[["Red",1],["Green",2],["Blue",6]],[["Green",2]]]},{"id":2,"contents":[[["Blue",1],["Green",2]],[["Green",3],["Blue",4],["Red",1]],[["Green",1],["Blue",1]]]},{"id":3,"contents":[[["Green",8],["Blue",6],["Red",20]],[["Blue",5],["Red",4],["Green",13]],[["Green",5],["Red",1]]]},{"id":4,"contents":[[["Green",1],["Red",3],["Blue",6]],[["Green",3],["Red",6]],[["Green",3],["Blue",15],["Red",14]]]},{"id":5,"contents":[[["Red",6],["Blue",1],["Green",3]],[["Blue",2],["Red",1],["Green",2]]]}]"#;

    #[test]
    fn example_parse() {
        let input = utils::read_to_lines(EXAMPLE_STR.as_bytes());

        let expected: Vec<Record> = serde_json::from_str(EXAMPLE_RECORD_JSON).unwrap();
        let actual: Vec<Record> = input.map(|line| parse_line(&line)).collect();

        assert_eq!(expected, actual);
    }
}
