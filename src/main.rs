use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::{thread, time};

fn read_file_content(filename: &str) -> Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn convert_board_to_array<'a>(contents: &'a str) -> Vec<Vec<&'a str>> {
    contents
        .split("\n")
        .map(|x: &str| x.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
}

fn convert_board_values_to_int(board: Vec<Vec<&str>>) -> Vec<Vec<Vec<i64>>> {
    board
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| {
                    y.split("")
                        .filter(|z| z != &"")
                        .map(|z| z.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>()
                })
                .collect::<Vec<Vec<i64>>>()
        })
        .collect::<Vec<Vec<Vec<i64>>>>()
}

fn print_board(board: Vec<Vec<Vec<i64>>>) {
    print!("{esc}c", esc = 27 as char);
    for row in board {
        for j in row {
            print!("{:?}\n", j);
        }
    }
}

fn get_agent_position(board: &Vec<Vec<Vec<i64>>>) -> (usize, usize, usize) {
    let mut agent_position = (0, 0, 0);

    let board_clone = board.clone();

    for (x, col) in board_clone.iter().enumerate() {
        for (i, row) in col.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if *value == 2 {
                    agent_position = (x, i, j)
                }
            }
        }
    }

    agent_position
}

fn is_movement_possible(board: Vec<Vec<Vec<i64>>>, position: (usize, usize, usize)) -> bool {
    let x = position.0;
    let y = position.1;
    let z = position.2;

    if x > board.len() - 1 {
        return false;
    }

    if z > board[x][y].len() - 1 {
        return false;
    }

    if board[x][y][z] != 1 && board[x][y][z] != 0 {
        return false;
    }

    return true;
}

fn compute_next_movement(
    board: Vec<Vec<Vec<i64>>>,
    current_position: (usize, usize, usize),
) -> String {
    let (current_x, current_y, current_z) = current_position;

    let mut next_up_position = (current_x, current_y, current_z);
    let mut next_down_position = (current_x, current_y, current_z);
    let mut next_left_position = (current_x, current_y, current_z);
    let mut next_right_position = (current_x, current_y, current_z);

    if current_x != 0 {
        next_up_position = (current_x - 1, current_y, current_z);
    }

    if current_x < board.len() - 1 {
        next_down_position = (current_x + 1, current_y, current_z);
    }

    if current_z != 0 {
        next_left_position = (current_x, current_y, current_z - 1);
    }

    if current_z < board[current_x][current_y].len() - 1 {
        next_right_position = (current_x, current_y, current_z + 1);
    }

    if next_up_position != current_position && is_movement_possible(board.clone(), next_up_position)
    {
        return "up".to_string();
    } else if next_left_position != current_position
        && is_movement_possible(board.clone(), next_left_position)
    {
        return "left".to_string();
    } else if next_right_position != current_position
        && is_movement_possible(board.clone(), next_right_position)
    {
        return "right".to_string();
    } else if next_down_position != current_position
        && is_movement_possible(board.clone(), next_down_position)
    {
        return "down".to_string();
    } else {
        return "no movement".to_string();
    }
}

fn move_agent_left(board: &mut Vec<Vec<Vec<i64>>>, agent_position: (usize, usize, usize)) {
    let (x, y, z) = agent_position;
    board[x][y][z + 1] = 1;
    board[x][y][z] = 2;
}
fn move_agent_up(board: &mut Vec<Vec<Vec<i64>>>, agent_position: (usize, usize, usize)) {
    let (x, y, z) = agent_position;
    board[x + 1][y][z] = 1;
    board[x][y][z] = 2;
}

fn move_agent_down(board: &mut Vec<Vec<Vec<i64>>>, agent_position: (usize, usize, usize)) {
    let (x, y, z) = agent_position;
    board[x - 1][y][z] = 1;
    board[x][y][z] = 2;
}

fn move_agent_right(board: &mut Vec<Vec<Vec<i64>>>, agent_position: (usize, usize, usize)) {
    let (x, y, z) = agent_position;
    board[x][y][z - 1] = 1;
    board[x][y][z] = 2;
}

fn resolve_maze(board: &mut Vec<Vec<Vec<i64>>>, agent_position: (usize, usize, usize)) {
    let (x, y, z) = agent_position;

    if board[x][y][z] == 0 || x == 0 {
        println!("You have reached the exit!");
        return;
    }

    let next_movement = compute_next_movement(board.clone(), agent_position);

    thread::sleep(time::Duration::from_secs(1));

    if next_movement == "up" {
        move_agent_up(board, (x - 1, y, z));
        print_board(board.clone());
        resolve_maze(board, (x - 1, y, z));
    } else if next_movement == "left" {
        move_agent_left(board, (x, y, z - 1));
        print_board(board.clone());
        resolve_maze(board, (x, y, z - 1));
    } else if next_movement == "right" {
        move_agent_right(board, (x, y, z + 1));
        print_board(board.clone());
        resolve_maze(board, (x, y, z + 1));
    } else if next_movement == "down" {
        move_agent_down(board, (x + 1, y, z));
        print_board(board.clone());
        resolve_maze(board, (x + 1, y, z));
    } else {
        return;
    }
}

fn main() -> std::io::Result<()> {
    let file_content = read_file_content("maze.txt")?;
    let board = convert_board_to_array(&file_content);
    let mut parsed_board = convert_board_values_to_int(board);
    let start_pos = get_agent_position(&parsed_board);

    resolve_maze(&mut parsed_board, start_pos);

    Ok(())
}
