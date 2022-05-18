use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;

pub struct Environment {
    pub map: Vec<Vec<i64>>,
    boundaries: (usize, usize),
}

impl Environment {
    pub fn new(map: &str) -> Environment {
        fn parse_to_array<'a>(contents: &'a str) -> Vec<Vec<i64>> {
            contents
                .split("\n")
                .map(|x: &str| {
                    x.chars()
                        .filter(|x| x.is_digit(10))
                        .map(|x| x.to_digit(10).unwrap() as i64)
                        .collect::<Vec<i64>>()
                })
                .collect::<Vec<Vec<i64>>>()
        }

        let parsed_map = parse_to_array(map);
        let x_boundarie = parsed_map.len() - 1;
        let y_boundarie = parsed_map[x_boundarie].len() - 1;

        Environment {
            map: parsed_map,
            boundaries: (x_boundarie, y_boundarie),
        }
    }

    pub fn new_from_file(filename: &str) -> Environment {
        fn read_file_content(filename: &str) -> Result<String> {
            let file = File::open(filename)?;
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents)?;
            Ok(contents)
        }

        fn parse_to_array<'a>(contents: &'a str) -> Vec<Vec<i64>> {
            contents
                .split("\n")
                .map(|x: &str| {
                    x.chars()
                        .filter(|x| x.is_digit(10))
                        .map(|x| x.to_digit(10).unwrap() as i64)
                        .collect::<Vec<i64>>()
                })
                .collect::<Vec<Vec<i64>>>()
        }

        let map = read_file_content(filename);
        let parsed_map = parse_to_array(map.unwrap().as_str());
        let x_boundarie = parsed_map.len() - 1;
        let y_boundarie = parsed_map[x_boundarie].len() - 1;

        Environment {
            map: parsed_map,
            boundaries: (x_boundarie, y_boundarie),
        }
    }

    pub fn get_map_boundaries(&self) -> (usize, usize) {
        self.boundaries
    }

    pub fn update_map(&mut self, from_position: (usize, usize), to_position: (usize, usize)) {
        let (from_x, from_y) = from_position;
        let (to_x, to_y) = to_position;

        self.map[from_x][from_y] = 1;
        self.map[to_x][to_y] = 2;
    }

    pub fn print_map_snapshot(&self) {
        print!("{esc}c", esc = 27 as char);
        for row in &self.map {
            print!("{:?}\n", row);
        }
    }
}
