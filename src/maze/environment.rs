pub struct Environment {
    pub map: Vec<Vec<i64>>,
    boundaries: (usize, usize),
}

impl Environment {
    pub fn new(map: Vec<Vec<i64>>, boundaries: (usize, usize)) -> Environment {
        Environment {
            map: map,
            boundaries: boundaries,
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
