# Collatz Sequence

## About
The Collatz sequence, also called the 3n + 1 problem, is the simplest impossible 
math problem. (But don’t worry, the program itself is easy enough for beginners.) 
From a starting number, n, follow three rules to get the next number in the sequence:
1. If n is even, the next number n is n / 2.
2. If n is odd, the next number n is n * 3 + 1.
3. If n is 1, stop. Otherwise, repeat.

It is generally thought, but so far not mathematically proven, that every starting 
number eventually terminates at 1. More information about the Collatz sequence can be 
found at https://en.wikipedia.org/wiki/Collatz_conjecture.

## Running the project
* Install Rust: [rustup.rs](https://rustup.rs/)
* Clone the repository locally:
  * `git clone https:://github.com/AndrewRosenfrisk/collatz`
  * `cd collatz`
* Build the project with `cargo build`
* Run the project with `cargo run`

Based on the project detailed in the "[Big Book of Small Python Projects](https://inventwithpython.com/bigbookpython/project12.html)"
