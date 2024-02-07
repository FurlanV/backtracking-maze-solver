use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;

pub struct Environment {
    pub map: Vec<Vec<usize>>,
    pub graph_map: HashMap<(usize, usize), Vec<(usize, usize)>>,
    pub finish_coords: Vec<(usize, usize)>,
    pub number_of_paths: usize,
    boundaries: (usize, usize),
}

impl Environment {
    pub fn new(filename: &str) -> Environment {
        fn read_file_content(filename: &str) -> Result<String> {
            let file = File::open(filename)?;
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents)?;
            Ok(contents)
        }

        fn parse_to_array<'a>(contents: &'a str) -> Vec<Vec<usize>> {
            contents
                .split("\n")
                .map(|x: &str| {
                    x.chars()
                        .filter(|x| x.is_digit(10))
                        .map(|x| x.to_digit(10).unwrap() as usize)
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        }

        let map = read_file_content(filename);
        let parsed_map = parse_to_array(map.unwrap().as_str());
        let x_boundarie = parsed_map.len() - 1;
        let y_boundarie = parsed_map[x_boundarie].len() - 1;
        let mut graph_map: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
        let mut n_paths = 0;
        let mut finish_coords = vec![];

        for (i, row) in parsed_map.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                let mut neighbors: Vec<(usize, usize)> = vec![];

                if *col == 0 {
                    finish_coords.push((i, j));
                }

                if *col == 0 || *col == 1 || *col == 2 {
                    let directions: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
                    for (di, dj) in directions.iter() {
                        let ni = (i as isize) + di;
                        let nj = (j as isize) + dj;
                        if ni >= 0
                            && nj >= 0
                            && ni <= x_boundarie as isize
                            && nj <= y_boundarie as isize
                        {
                            if parsed_map[ni as usize][nj as usize] == 1
                                || parsed_map[ni as usize][nj as usize] == 0
                            {
                                neighbors.push((ni as usize, nj as usize));
                                n_paths += 1;
                            }
                        }
                    }
                    graph_map.insert((i, j), neighbors);
                }
            }
        }

        Environment {
            map: parsed_map,
            boundaries: (x_boundarie, y_boundarie),
            graph_map: graph_map,
            number_of_paths: n_paths,
            finish_coords: finish_coords,
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
