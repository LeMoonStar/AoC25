# Advent of Code 2025

[![About](https://img.shields.io/badge/Advent%20of%20Code-2025-brightgreen?style=flat-square)](https://adventofcode.com/2025/about)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg?style=flat-square)](https://en.wikipedia.org/wiki/Rust_(programming_language))
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](https://mit-license.org/)
![Days completed](https://img.shields.io/badge/Days%20completed-3-red?style=flat-square)
![Stars](https://img.shields.io/badge/Stars-6-yellow?style=flat-square)

My Advent of Code 2025 Solutions.

## Usage

There are multiple ways to run my solutions, the easiest and most comfortable one is the `auto` command:  
It automatically downloads your input. For this it requires you to provide your Advent of Code session id, which you can find in the websites cookies after logging in.  
Simply provide the session token by setting the `AOC_SESSION` environment variable or using the -s argument:  
`AOC_SESSION=XXXMYSESSION ./aoc25 [DAY] auto` or `./aoc25 [DAY] auto -s XXXMYSESSION`.  
In this example, the environment variable for the AoC session is set using `export AOC_SESSION=XXXMYSESSION`, so I can run the command without specifying the session token again:  
![auto command in action](./images/auto.png)  

If you don't want to automatically download the input, you can also use the `run` command, which uses a locally stored file or the stdin input:  
`./aoc25 [DAY] run -f my_input.txt`:  
![run command in action](./images/run.png)  

If you just want to run the day's example, simply use the `test` command, as this project already includes the examples:
`./aoc25 [DAY] test`:  
![test command in action](./images/test.png)  

## Compiling

This project uses `Cargo`, so compiling is pretty easy:  
`cargo build --release`  
The resulting binary can be found at `./targets/release/aoc25`. You can also directly run the project using `cargo run --release [arguments for aoc25]`  
the `--release` option is not required, but it results in better performance.

## Check out other AoC25 solutions

> TODO