# RustyBase

![Rust](https://github.com/InfernapeXavier/DataBase/workflows/Rust/badge.svg?branch=master&event=push)

> A database written in Rust without any unsafe code!

### Build Steps

- `cargo build` will build the project in debug mode [1]
- `cargo build --release` will build the project with optimizations on <- _This takes almost twice the amount of time to build_
- The `--release` flag [2] will use the optimized build to run and can be used with all `cargo` commands
- `cargo run` will run the project. But there isn't really anything in the main function. Everything is written in the form of tests.
- `cargo test` will launch the tests one by one (_in alphabetical order_), and only show the result i.e. _pass_ or _fail_
- `cargo test -- --nocapture` will also launch tests one by one (_in alphabetical order_) but will also show the outputs/errors
- `cargo test -- --nocapture <test-name>` will launch just the specified test and will show outputs/errors [3]
- Sample inputs for the test are in the _sampleinputs_ file and are also included as comments in each test
- There is a make-file but because cargo does most things it's not really required. I only used it for additional cleaning of my scratch files.
- The make-file uses Cargo-Make which is an external crate (library) but I had it locally installed and it's not part of the project dependencies
- The only external crate is _LALRPOP_ which is the parser. However, _LALRPOP_ has about 95 dependencies, which is the reason for the large compile times
- _LALRPOP_ has it's own lexer and because the creator's intention was to simplify Bison and Flex, it has a lot of macros and shortcuts that make the parser very easy to write
- I've included the TPCH 10MB files in the _tpch/_ folder
- All file paths are built in main as the Rust `PATH` trait that handles UNIX to Windows conversions
- For paths, Rust uses the folder with _cargo.toml_ as the source

#### Footnotes

1. Cargo also has a built-in linter that can be called using `cargo clippy`, but due to _LALRPOP_ there are a lot of warning (all of which are _help_ hints)
2. The release version that cargo builds is not meant to be built over and over again and is very slow. It should not be used if the files are going to be changed
3. To test the release version use `cargo test --release -- --nocapture <test-name>`
