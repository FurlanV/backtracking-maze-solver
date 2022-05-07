use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("maze.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let formated_maze_values = contents
        .split("\n")
        .map(|x: &str| x.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let maze = formated_maze_values.iter().map(|x| x.iter().map(|y| y.parse::<i64>().unwrap()).collect::<Vec<i64>>()).collect::<Vec<Vec<i64>>>();

    for line in maze {
        println!("{:?}", line);
    }

    // let maze_values = maze_values
    //     .into_iter()
    //     .map(|x: &str| match x {
    //         "" => x.split("").collect::<Vec<&str>>(),
    //         " " => x.split(" ").collect::<Vec<&str>>(),
    //         _ => x.split("").collect::<Vec<&str>>(),
    //     })
    //     .map(|x| x.into_iter().filter(|y| y != &"").collect::<Vec<&str>>())
    //     .collect::<Vec<Vec<&str>>>();

    // for line in maze_values {
    //     println!("{:?}", line);
    // }

    // for values in maze_values {
    //     let current_value;
    //     if values.contains(" ") {
    //         current_value = values.split(" ").collect::<Vec<&str>>();
    //     } else {
    //         current_value = values.split("").collect::<Vec<&str>>();
    //     }
    //     maze.push(current_value);
    // }

    // maze = maze
    //     .into_iter()
    //     .map(|x| x.into_iter().filter(|x| x != &"").collect::<Vec<&str>>())
    //     .collect::<Vec<Vec<&str>>>();

    // for line in maze {
    //     println!("{:?}", line);
    // }

    Ok(())
}
