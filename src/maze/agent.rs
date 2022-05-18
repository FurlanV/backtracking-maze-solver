use super::environment::Environment;

pub struct Agent {
    pub current_position: (usize, usize),
    pub last_position: (usize, usize),
    next_planned_movement: (usize, usize),
    n_of_movements: usize,
}

impl Agent {
    pub fn new(environment: &Environment) -> Self {
        fn get_initial_position(board: &Vec<Vec<i64>>) -> (usize, usize) {
            let mut agent_position = (0, 0);
            let board_clone = board.clone();
            for (i, col) in board_clone.iter().enumerate() {
                for (j, row) in col.iter().enumerate() {
                    if *row == 2 {
                        agent_position = (i, j);
                    }
                }
            }
            agent_position
        }

        let initial_position = get_initial_position(&environment.map);

        Self {
            current_position: initial_position,
            last_position: initial_position,
            next_planned_movement: initial_position,
            n_of_movements: 0,
        }
    }

    fn look(&self, position: (usize, usize), environment: &Environment) -> i64 {
        let (x, y) = position;
        return environment.map[x][y];
    }

    pub fn walk(&mut self, position: (usize, usize)) {
        self.last_position = self.current_position;
        self.current_position = position;
        self.n_of_movements += 1;
    }

    pub fn evaluate_next_movement(&mut self, environment: &Environment) {
        let (x_boundarie, y_boundarie) = environment.get_map_boundaries();
        let (x, y) = self.current_position;
        let mut decision = false;

        if x > 0 && !decision {
            let up = (x - 1, y);
            if self.look(up, &environment) == 1 {
                self.next_planned_movement = up;
                decision = true;
            }
        }
        if y > 0 && !decision {
            let left = (x, y - 1);
            if self.look(left, &environment) == 1 {
                self.next_planned_movement = left;
                decision = true;
            }
        }
        if y < y_boundarie && !decision {
            let right = (x, y + 1);
            if self.look(right, &environment) == 1 {
                self.next_planned_movement = right;
                decision = true;
            }
        }

        if x < x_boundarie && !decision {
            let down = (x + 1, y);
            if self.look(down, &environment) == 1 {
                self.next_planned_movement = down;
            }
        }
    }

    pub fn get_next_movement(&self) -> (usize, usize) {
        return self.next_planned_movement;
    }
}
