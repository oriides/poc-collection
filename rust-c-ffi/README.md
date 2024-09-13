# Rust C Foreign Function Interface (FFI) Example

Rust allows the inclusion of program-code written in other languages via foreign function interfaces (FFIs).

Projects like [Rust-for-Linux](https://github.com/Rust-for-Linux) make heavy use of this in order to build Rust APIs for the Linux-Kernel that enable the development of kernel code using the Rust programming language.

This repo acts as an example on how to include a C file into a Rust project.

At the center of this C FFI stands the [cc-rs crate](https://crates.io/crates/cc) which is a compile-time library to compile C/C++/assembly into a Rust library/application.

See the [README.md for the cc-rs crate](https://github.com/rust-lang/cc-rs/blob/main/README.md) for more usage instructions (e.g platform support).

**⚠ BEWARE:** Rust FFIs are not beginner friendly and require pretty extensive knowledge of both programming languages in order to be used in real use cases. If you want to use FFIs you have to deal with `unsafe`, which means **you are responsible** for a correct/safe/sound program.

## Trying out the example

Run the following command to build and run the example program

```bash
cargo run
```

Output should look something like this:

```stdout
my_struct->x = 2
my_struct->x = 6
my_struct->x = 2
my_struct->x = 6
```

## Repo Structure

```text
.
├── src/
│   ├── include/
│   │   ├── my_struct_lib.c
│   │   └── my_struct_lib.h
│   ├── bindings.rs
│   ├── main.rs
│   └── wrapper.rs
├── build.rs
└── Cargo.toml
```

Let me point out the most important bits:

- `Cargo.toml` is where project information and dependencies are tracked. Notice the `cc` build-dependency as well as the `links = "my_struct"` and `build = "build.rs"` declarations.
- `build.rs` contains the custom build instructions for the rust compiler. Notice the `cc::Build::new().file("src/include/my_struct_lib.c").compile("my_struct");` command which tells the rust compiler to build the `src/include/my_struct_lib.c` file.
- `main.rs` is where the rust main function lives. Here we demonstrate how to use the C Struct in Rust code.
- `bindings.rs` contains the bindings to the C struct and its functions. For small projects this file can be created manually, for larger projects I strongly recommend using [bindgen](https://crates.io/crates/bindgen) to generate these bindings automatically.
- `wrapper.rs` contains a wrapper struct in idiomatic Rust around the C struct we are linking to. We are also including the bindings contained in `bindings.rs`
- `my_struct_lib.c` & `my_struct_lib.h` contains an implementation of a struct in C code. We want to use this implementation in our Rust code.
