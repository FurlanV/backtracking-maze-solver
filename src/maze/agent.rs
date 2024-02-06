use std::{thread, time};

use super::environment::Environment;

#[derive(PartialEq, Debug)]
enum MovementPossibilities {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    DeadEnd,
}
enum SolutionPossibilities {
    Found,
    NotFound,
    Backtrack,
    NoSolution,
}

pub struct Agent {
    pub current_position: (usize, usize),
    pub visited_positions: Vec<(usize, usize)>,
    pub solution_path: Vec<(usize, usize)>,
}

impl Agent {
    pub fn new(environment: &Environment) -> Self {
        fn get_initial_position(board: &Vec<Vec<usize>>) -> (usize, usize) {
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
            visited_positions: vec![initial_position],
            solution_path: vec![initial_position],
        }
    }

    fn look_around(&mut self, environment: &Environment) -> Vec<MovementPossibilities> {
        let directions: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        let mut possibilities = Vec::new();

        for (di, dj) in directions.iter() {
            let ni = (self.current_position.0 as isize) + di;
            let nj = (self.current_position.1 as isize) + dj;
            if ni >= 0
                && nj >= 0
                && ni <= environment.get_map_boundaries().0 as isize
                && nj <= environment.get_map_boundaries().1 as isize
            {
                if environment.map[ni as usize][nj as usize] == 3 {
                    possibilities.push(MovementPossibilities::DeadEnd);
                } else if environment.map[ni as usize][nj as usize] == 1
                    || environment.map[ni as usize][nj as usize] == 0
                {
                    if (*di as isize, *dj as isize) == (-1, 0) {
                        possibilities.push(MovementPossibilities::MoveRight);
                    } else if (*di as isize, *dj as isize) == (0, -1) {
                        possibilities.push(MovementPossibilities::MoveUp);
                    } else if (*di as isize, *dj as isize) == (1, 0) {
                        possibilities.push(MovementPossibilities::MoveLeft);
                    } else if (*di as isize, *dj as isize) == (0, 1) {
                        possibilities.push(MovementPossibilities::MoveDown);
                    }
                }
            }
        }

        possibilities
    }

    pub fn explore(&mut self, environment: &mut Environment) {
        let is_solution = self.evaluate_solution(environment);

        match is_solution {
            Some(SolutionPossibilities::Found) => {
                println!("Solution found! - {:?}", self.solution_path);
                return;
            }
            Some(SolutionPossibilities::NotFound) => {
                let path_possibilities: Option<&Vec<(usize, usize)>> =
                    environment.graph_map.get(&self.current_position);

                let mut next_position = (0, 0);

                for possibility in path_possibilities.unwrap() {
                    if !self.visited_positions.contains(possibility) {
                        next_position = *possibility;
                        break;
                    }
                }

                if next_position == (0, 0) {
                    self.backtrack();
                } else {
                    self.visited_positions.push(next_position);
                    self.current_position = next_position.clone();
                    self.solution_path.push(self.current_position);

                    environment.update_map(self.current_position, next_position);
                    thread::sleep(time::Duration::from_millis(500));
                    environment.print_map_snapshot();
                }
            }
            Some(SolutionPossibilities::Backtrack) => {
                self.backtrack();
            }
            Some(SolutionPossibilities::NoSolution) => {
                println!("No solution found!");
                return;
            }
            None => {
                println!("No solution found!");
                return;
            }
        }

        self.explore(environment);
    }

    fn evaluate_solution(&mut self, environment: &Environment) -> Option<SolutionPossibilities> {
        if environment.finish_coords.contains(&self.current_position){
            return Some(SolutionPossibilities::Found);
        } else {
            let possibilities: Vec<MovementPossibilities> = self.look_around(environment);
            let mut dead_end: bool = false;

            dead_end = possibilities
                .iter()
                .all(|p| *p == MovementPossibilities::DeadEnd);

            if dead_end {
                return Some(SolutionPossibilities::Backtrack);
            } else {
                if self.visited_positions.len() == environment.number_of_paths {
                    return Some(SolutionPossibilities::NoSolution);
                }

                return Some(SolutionPossibilities::NotFound);
            }
        }
    }

    fn backtrack(&mut self) {
        let last_position = self.solution_path.pop().unwrap();
        self.current_position = last_position;
    }
}
