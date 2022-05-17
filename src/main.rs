use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::{thread, time};

mod maze;

use maze::agent::Agent;
use maze::environment::Environment;

fn read_file_content(filename: &str) -> Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn convert_board_to_array<'a>(contents: &'a str) -> Vec<Vec<i64>> {
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

fn get_agent_position(board: &Vec<Vec<i64>>) -> (usize, usize) {
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

fn resolve_maze(environment: &mut Environment, agent: &mut Agent) {
    if environment.map[agent.current_position.0][agent.current_position.1] == 0 {
        return;
    }

    thread::sleep(time::Duration::from_secs(1));

    agent.evaluate_next_movement(&environment);
    let next_position = agent.get_next_movement();
    agent.walk(next_position);
    environment.update_map(agent.last_position, agent.current_position);
    environment.print_map_snapshot();

    resolve_maze(environment, agent);
}

fn main() -> std::io::Result<()> {
    let file_content = read_file_content("maze.txt")?;
    let parsed_board = convert_board_to_array(&file_content);

    let agent_initial_position = get_agent_position(&parsed_board);
    let x_boundarie = parsed_board.len() - 1;
    let y_boundarie = parsed_board[x_boundarie].len() - 1;

    let mut environment = Environment::new(parsed_board, (x_boundarie, y_boundarie));
    let mut agent = Agent::new(agent_initial_position);

    resolve_maze(&mut environment, &mut agent);

    Ok(())
}
