## Backtracking maze solver

Simple backtracking algorithm to solve a maze.
The maze environmet is represented by a matrix where each cell containing '1' as a value represents a possible movement. the number '2' represents the agent who is trying to solve this maze. Number '3' represent the walls. The agent cannot jump walls or move diagonally though the maze.
The algorithm has only one call to the function resolve_maze. From there, the function call itself until the maze is solved. This strategy is called backtracking.

### How it works

