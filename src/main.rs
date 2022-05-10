use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;

fn convert_to_array_board<'a>(contents: &'a str) -> Vec<Vec<Vec<&'a str>>> {
    contents
        .split("\n")
        .map(|x: &str| {
            x.split(" ")
                .map(|x| x.split("").collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>()
        })
        .collect::<Vec<Vec<Vec<&str>>>>()
}

fn read_file_content(filename: &str) -> Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> std::io::Result<()> {
    let file_content = read_file_content("maze.txt")?;
    let board = convert_to_array_board(&file_content);

    println!("{:?}", board);

    Ok(())
}
