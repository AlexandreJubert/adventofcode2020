![Rust](https://github.com/AlexandreJubert/adventofcode2020/workflows/Rust/badge.svg)

# Advent Of Code 2020

Welcome to this repository containing all the code I've used to answer the 2020 Advent of Code challenges !

This year I've decided to use `Rust` to complete the challenges, so you won't find the most optimal nor the most concise code, not even the most documented code. I've took this year `AoC` as an opportunity to continue learning `Rust` and deepen my understanding of this language !

# How to compile

First of all you'll need a rust toolchain, the easiest way is too install [rustup](https://rustup.rs/) and get the latest `Rust` toolchain for your OS. In my case since I'm using Windows, my command line would be

```
rustup toolchain install stable-msvc
```

This will install the latest stable `Rust` toolchain using the latest Microsoft Visual C compiler.

Next you can clone this repository with 

```
git clone https://github.com/AlexandreJubert/adventofcode2020.git
```

Finally to compile just open a terminal inside the `adventofcode2020/` folder you just cloned and run

```
cargo build
```

# How to run

Just use `cargo run` to execute the main function. The result will depend of which day I'm currently working on, most days it'll be empty. But feel free to modify `src/main.rs` to use any day function I've written inside the differents day modules.

# Documentation

The code is not really documented, but you can try to use 

```
cargo doc --open
```

To generate and open the crate documentation.

# Unit Tests

To run the unit tests, run the following command:

``` 
cargo test
```

All the tests are also ran by `Github Actions` on every submit.
