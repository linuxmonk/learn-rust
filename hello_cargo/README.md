### Using Cargo

### Create a new package

Command is `cargo new <package_name>`

Example: `cargo new hello_cargo`

Creates these files -

```
.
├── Cargo.toml
└── src
    └── main.rs
```

### Cargo.toml is the configuration file for the module/package

```
[package]
name = "hello_world"
version = "0.1.0"
authors = ["sai"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

### Building a package using cargo

Build using `cargo build`. This generates `<package-dir>/target/debug/...`

### Running a binary using cargo

Run using `cargo run`.
