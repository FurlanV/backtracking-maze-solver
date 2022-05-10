use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;

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

fn convert_board_values_to_int(board: &Vec<Vec<&str>>) -> Vec<Vec<i64>> {
    board
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

fn print_board(board: &Vec<Vec<i64>>) {
    for (i, row) in board.into_iter().enumerate() {
        if i != 0 {
            for j in row.into_iter() {
                print!("{} ", j);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let file_content = read_file_content("maze.txt")?;
    let board = convert_board_to_array(&file_content);
    let parsed_board = convert_board_values_to_int(&board);
    print_board(&parsed_board);

    Ok(())
}
