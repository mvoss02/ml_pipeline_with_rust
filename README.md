# ML Pipeline with Rust

Building my first project with Rust. It will be a simple ML model which will be served via a REST API. It is mainly for me persoanlly to learn how to setup Rust projects. I will use it as a blueprint for further projects.

## Install Rust Tools

1. Rust Compiler + Cargo (packet manager - similar to poetry, conda, and uv)
   Rust can be isntalled on **macOS, Linux or any other Unix-like OS** with the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Further information: https://www.rust-lang.org/tools/install

2. Check if Rust + Cargo were successfully installed

```bash
rustc --version
```

```bash
cargo --version
```

3. Install _rust analyzer_ for VSCode
   A tool which highlights potential issues, without having to compile the program every single time.

   Go to `Extensions > type "rust-analyzer" > Install` extension

Further Information: https://rust-analyzer.github.io/

## Create a Project Scaffold with Cargo

Cargo allows to create a baseline project structure coming with useful tools and setup to get started, by running:

```bash
cargo new <DESIRED_DIR_NAME>
```

## Compile and Run Your First Rust Program

Run main.rs program, which will print "hello-world!":

```rust
fn main() {
    println!("Hello, world!");
}
```

From within the given directory run:

```bash
# Compiles and runs the program
cargo run <FILE_NAME>
```

Note that the file name is not mandatory. Cargo by default looks for the src/main.rs directory. **It first compiles the file and then runs it.**

Alternatively, one can also run:

```bash
rustc src/main.rs
```

However, the former is more convinient. Rustc is the Rust compiler itself and is typically used for compiling single-file programms or when you need low-level control over the compilation process. It does not inherently manage projects or dependencies. Cargo, on the other hand, is Rust's build system and package manager. It’s designed to handle projects with multiple files, dependencies, and configurations, which are common in real-world Rust development.

Running a program creates a binary in the `target/` directory. A binary in Rust is an executable file created by compiling code with a main() function. It’s the program you can run directly on your computer, like a command-line tool or app. No need for rust compiler or cargo anymore, It’s used to execute your Rust program and perform tasks, such as printing output or running a game, after being built from source code with cargo build or cargo run. The binary which is outputed by this process can be run directly, without having to install all dependencies (like in Python, for example). In order to run it, it simply needs to be invoked via the command-line:

```bash
# The file name will be similar/equal to the name
# of the parent directory where you ran your script
# (main.rs) originally from
./target/debug/<FILE_NAME>
```

Note, the file is stored in `./target/debug` for faster execution. If you want to create it for relaese add the `--release` flag to your run command:

```bash
# Create binary file optimized for release
cargo run --release
```

## Data

The Boston Housing Prices dataset can be downloaded from, if you dont ant to download it dynamically in the code:

https://github.com/selva86/datasets/blob/master/BostonHousing.csv

I placed it in the folder ./house-price-predictor/data/ and downlaoded it dynamically form thsi link: https://raw.githubusercontent.com/selva86/datasets/master/BostonHousing.csv

## Install Library in Rust

Features ensure that we only install the necessary functionality, not anything which we would not use.

```bash
cagro add <LIBRARY_NAME> --features <SUB_FEATURE_NAME>
```
