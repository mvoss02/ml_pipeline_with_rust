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

   Go to Extensions > type "rust-analyzer" > Install extension

Further Information: https://rust-analyzer.github.io/
