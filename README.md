# rs-annealing

This project is a Rust-based solution for a competitive programming problem, likely from a platform like AtCoder. It uses a simulated annealing algorithm to find the optimal schedule for a series of contests to maximize a score.

## Description

The core of the project is a simulated annealing algorithm that iteratively seeks to improve a solution by exploring the solution space. It starts with an initial solution and makes random modifications, accepting them based on a temperature parameter that decreases over time. This allows the algorithm to escape local optima and find a globally optimal solution.

## Project Structure

- `src/main.rs`: The main program that implements the simulated annealing solver. It reads the problem input, initializes a solution, and iteratively improves it until a time limit is reached.
- `src/b.rs`: A utility program, likely used for scoring a given solution against the problem's constraints.
- `data/`: This directory is intended to hold input files for the problem.
- `Cargo.toml`: The Rust package manager configuration file, specifying dependencies like `proconio` for input processing and `rand` for random number generation.

## Usage

To compile and run the project, you will need to have Rust and Cargo installed.

1. **Build the project:**
   ```bash
   cargo build --release
   ```

2. **Run the solver:**
   Pipe the input data to the program:
   ```bash
   cargo run --release --bin rs-annealing < data/input.txt
   ```
   The output will be the best solution found within the time limit.

3. **Run the scorer (optional):**
   If you have a solution file, you can use the `b` utility to score it:
   ```bash
   cargo run --release --bin b < data/solution.txt
   ```
This will compute and print the score for the provided solution.