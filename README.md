# Minigrep
This is the recreation of the [chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
from the [Rust book](https://doc.rust-lang.org/book/).

*Update: Read commits to go to code as in chapter 12, the code got updated after reading chapter 13.*

## Requirements
Rust >= 1.34.0

## How to use

```sh
cargo run body poem.txt

# To get return data to a file.
cargo run body poem.txt > output.txt

# Case insensitive
CASE_INSENSITIVE=1 cargo run BoDy poem.txt
```
