# Maze Solver in Rust

## Overview

This Maze Solver is a Rust-based implementation designed to navigate and solve mazes using a backtracking algorithm. The maze is modeled as a matrix, where integers represent possible movements, the agent's position, and walls. The solver uses a single recursive function call to explore the maze until it finds a solution or determines one is not possible.

## Features

- **Maze Representation**: A matrix where:
  - `1` indicates a traversable cell.
  - `2` represents the agent's starting position.
  - `3` denotes walls that the agent cannot bypass or move diagonally past.

- **Environment Module**: Constructs the maze from a file, gathers essential details such as the maze's exit coordinates and the number of viable paths, and creates two maps:
  - An adjacency list for pathfinding.
  - A matrix for visualizing the agent's movements.

- **Agent Module**: Uses backtracking to navigate through the maze, relying on the maps and information prepared by the Environment module to seek out a solution.

## How It Works

1. **Environment Setup**: The environment reads the maze from a file, identifying start and end points, and obstacles, then prepares an adjacency list and a matrix map for navigation and visualization.

2. **Agent Navigation**: Starting at the specified point, the agent explores the maze, using backtracking to maneuver through paths, avoid walls, and attempt to find the exit.

3. **Solution Verification**: The agent uses information like the end coordinates and possible paths to verify if a solution is feasible.

## Requirements

- Rust programming environment setup.
- A maze file formatted with `1` for paths, `2` for the agent's starting point, and `3` for walls.

## Usage

1. Format your maze file according to the specified conventions.
2. Compile and execute the Rust program with the maze file as input.
3. The solver will output the solved path if one exists, along with a visualization of the agent's path through the maze.

## Limitations

- The agent cannot jump over walls or move diagonally.
- The algorithm assumes the maze is solvable with the given paths; it will report no solution if otherwise.

## Conclusion

This project demonstrates a practical application of the backtracking algorithm in Rust for solving mazes. By splitting the problem into an Environment for maze setup and an Agent for navigation, this implementation offers an efficient and effective solution to maze-solving puzzles.
