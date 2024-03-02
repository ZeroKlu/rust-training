### Install Commands ###

```winget install Rustlang.Rustup```

```winget install Microsoft.VIsualStudio.2022.BuildTools```

---

### Rust Commands ###

```rustc --version```

```rustup doc```

```rustc hello_world.rs``` then
```.\hello_world.exe```

```rustfmt hello_world.rs```

---

### Cargo Commands ###

```cargo --version```

```cargo new hello_world [--vcs=git]```<br>
optional param overrides behavior and adds .gitignore in an existing Git repo

Running ```cargo new``` creates Cargo.toml
```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

...

[dependencies]
```

```cargo build``` then
```.\target\debug\hello_world.exe```

```cargo run``` compiles and executes

```cargo check``` verifies the code will compile

```cargo build --release```



