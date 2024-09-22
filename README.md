# Advent of Code
Solutions for [Advent of Code](https://adventofcode.com/).

## Overview
This repository provides prompts in a `README` under each day's
directory along with my personalized input.  This means that all information
provided by Advent of Code is available in this repo; you do not need to
navigate the website in order to inspect my solutions.

My solutions will also, when run, provide the answers to both parts of a day's
question using my personalized input.  If you want to use a different input,
replace the contents of the `input.txt` file for that day with your input
string.

## Languages and Getting Answers
### Rust
All Rust solutions will be located under a `rust` directory.  This directory is
the project root for that day and each project utilizes `Cargo` for management.
This requires you to have `Cargo` installed to run the solutions. Navigate to
the project root and issue the commands you want.

- To run the solution and print the answers:

    ```sh
    cargo run --release
    ```
- To run the tests for the solution (which are provided by Advent of Code's prompts):

    ```sh
    cargo test
    ```
- To view documentation for the solution:

    ```sh
    cargo doc --open
    ```
    - Note that if you are using Safari, it prevents loading of the CSS file for
      the generated documentation. You must enable the `Show features for web
      developers` option and restart. You might also need to enable the `Disable
      local file restrictions` option in the Developer menu.