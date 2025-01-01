# Rust Modular System: Project Structure and File Organization

This repository demonstrates the modular organization of Rust code, breaking down the project into different modules, crates, and files. Rust’s module system is an essential aspect of structuring a large Rust codebase, and this project showcases how to organize, import, and use modules efficiently.

## File Organization

The project is divided into three main directories, each showcasing different ways to organize and use modules:

### 1. [include-01](./include-01/)

This section demonstrates how to include and use basic modules within a project.

```bash
include-01
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── module-01
│   │   ├── array.rs
│   │   └── sign.rs
│   └── src
│       └── main.rs
```

In this directory, the `main.rs` file directly references files inside the `module-01` folder. The modules are included and used directly without any crate setup.

#### Key Concepts

- Modules are directly included in `main.rs`.
  
- No external crate definition is required.

### 2. [library-02](./library-02)

This section demonstrates how to structure a library crate, where the code is encapsulated inside a library and can be imported into the main project.

```bash
library-02
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── local_lib
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── lib.rs
│   │       └── message.rs
│   └── src
│       └── main.rs
```

Here, `local_lib` is a library crate defined within the project. It contains its own `Cargo.toml` and source files, while the main project imports and uses it as an external dependency.

#### Key Concepts

- `local_lib` is a separate crate with its own `Cargo.toml` and source files.
- The `main.rs` file in the root project imports the library crate using `use`.

### 3. [moduler-03](./moduler-03/)

This section demonstrates organizing code into multiple modules within a single crate.

```bash
moduler-03
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       ├── basic
│       │   ├── mod.rs
│       │   └── sign.rs
│       └── main.rs
```

In this setup, the project is split into multiple submodules under the `basic` module, with `mod.rs` used to define and expose the `sign.rs` module.

#### Key Concepts

- `mod.rs` is used to declare submodules.
- The `main.rs` file imports the `basic::sign` module and calls functions defined inside `sign.rs`.

## Rust File Import and Module System

Rust uses a powerful and flexible module system that organizes code into modules, submodules, and crates. Here’s how it works:

### 1. **Modules**

A module is a collection of related functions, structs, traits, or other items. It allows you to organize your code logically. For example

- `mod basic;` declares a submodule named `basic` in `main.rs`.
- The `mod.rs` file inside a directory serves as the entry point for the module. For instance, `basic/mod.rs` declares other modules like `sign.rs`.

### 2. **Crates**

A crate is a package of Rust code. It can either be a library or a binary.

- `local_lib` in `library-02` is a separate crate with its own `Cargo.toml`.
- Each folder with a `Cargo.toml` represents a crate in Rust.

### 3. **Importing Modules**

Rust uses the `use` keyword to import modules, submodules, or crates. In this project

- In `include-01/src/main.rs`, the modules `array.rs` and `sign.rs` are directly imported using `use module_01::sign`.
  
- In `moduler-03/src/main.rs`, we use `use basic::sign;` to import the `sign` module from `basic`.

### 4. **Visibility**

Rust modules are private by default. You need to explicitly use the `pub` keyword to make items accessible outside the module.

- For example, to use functions in `sign.rs`, you need to declare them as `pub`:

```rust
  pub fn example_function() {
      println!("This is an example function.");
  }
```

## Run 

```bash
cargo run --manifest-path=include-01/Cargo.toml
cargo run --manifest-path=library-02/Cargo.toml
cargo run --manifest-path=moduler-03/Cargo.toml
```