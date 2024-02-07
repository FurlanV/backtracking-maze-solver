mod maze;

use maze::agent::Agent;
use maze::environment::Environment;

fn main() -> std::io::Result<()> {

    let mut environment = Environment::new("maze.txt");
    let mut agent = Agent::new(&environment);

    agent.explore(&mut environment);

    Ok(())
}
