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

fn resolve_maze(environment: &mut Environment, agent: &mut Agent) {
    if environment.map[agent.current_position.0][agent.current_position.1] == 0 {
        return;
    }

    thread::sleep(time::Duration::from_millis(500));

    agent.evaluate_next_movement(&environment);
    let next_position = agent.get_next_movement();
    agent.walk(next_position);
    environment.update_map(agent.last_position, agent.current_position);
    environment.print_map_snapshot();

    resolve_maze(environment, agent);
}

fn main() -> std::io::Result<()> {
    let file_content = read_file_content("maze.txt")?;
    //let agent_initial_position = get_agent_position(&parsed_board);

    let mut environment = Environment::new(&file_content);
    let mut agent = Agent::new(&environment);

    resolve_maze(&mut environment, &mut agent);

    Ok(())
}
