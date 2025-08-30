# Learning Rust 🚀

This repository is my Rust learning space, where I practice core concepts using simple code examples with explanatory comments. The aim is to gradually build a strong foundation in Rust while keeping the notes clear, concise, and beginner-friendly.

## Rust Facts
- Created by Graydon Hoare at Mozilla in 2010.
- Rust is a general purpose language for creating safe, secure, and scalable applications.
- Originally designed as systems programming language.
- The [Borrow Checker](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) is an unique feature within Rust.
- Rust Terminology
  - crate → compilation unit in Rust, can be a binary (executable), library, or an external dependency.
  - cargo 
    - cargo tool → Rust's package manager
    - cargo.toml → manifest and configuration file for Rust
    - cargo.lock → record of all dependencies with their specific versions

## 🦀 Why Rust?

Rust is known for:
- ⚡ **Performance** → Fast and memory-efficient, suitable for critical services, embedded devices, and easily integrated with other languages.
- 🔒 **Reliability** → Rich type system and ownership model ensure memory and thread safety, reducing bugs at compile-time.
- 🛠️ **Productivity** → Comprehensive documentation, a compiler committed to providing great diagnostics, and advanced tooling including package manager and build tool ([Cargo](https://github.com/rust-lang/cargo)), auto-formatter ([rustfmt](https://github.com/rust-lang/rustfmt)), linter ([Clippy](https://github.com/rust-lang/rust-clippy)) and editor support ([rust-analyzer](https://github.com/rust-lang/rust-analyzer)).

## 📂 Structure
- `variables/` → Basics of variables, mutability, shadowing
- `ownership/` → Ownership, borrowing, references
- `functions/` → Function definitions, return types, closures
- `structs/` → Structs, methods, associated functions
- `enums/` → Enums, pattern matching
- `collections/` → Vectors, HashMaps, Strings

## ▶️ Running examples
Navigate to the concept folder and run:
```bash
cargo run
```

## 📚 References
- [The Rust Programming Language](https://doc.rust-lang.org/book/)  
- [Learn Rust in a Month of Lunches](https://www.manning.com/books/learn-rust-in-a-month-of-lunches/)  
- [Programming With Rust](https://www.oreilly.com/library/view/programming-with-rust/9780137889754/)
- [Rust Tutorial for Beginners by Harkirat Singh - Youtube](https://www.youtube.com/watch?v=qP7LzZqGh30&t=4110s&ab_channel=HarkiratSingh)