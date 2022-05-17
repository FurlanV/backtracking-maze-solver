use super::environment::Environment;

pub struct Agent {
    pub current_position: (usize, usize),
    pub last_position: (usize, usize),
    next_planned_movement: (usize, usize),
    n_of_movements: usize,
}

impl Agent {
    pub fn new(current_position: (usize, usize)) -> Self {
        Self {
            current_position: current_position,
            last_position: (0, 0),
            next_planned_movement: current_position,
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
        let up = (x - 1, y);
        let down = (x + 1, y);
        let left = (x, y - 1);
        let right = (x, y + 1);

        if x > 0 && x < x_boundarie {
            if self.look(up, &environment) == 1 {
                self.next_planned_movement = up;
            }
        } else if x > 0 && x < x_boundarie {
            if self.look(down, &environment) == 1 {
                self.next_planned_movement = down;
            }
        } else if y > 0 && y < y_boundarie {
            if self.look(left, &environment) == 1 {
                self.next_planned_movement = left;
            }
        } else if y > 0 && y < y_boundarie {
            if self.look(right, &environment) == 1 {
                self.next_planned_movement = right;
            }
        }
    }

    pub fn get_next_movement(&self) -> (usize, usize) {
        return self.next_planned_movement;
    }
}
