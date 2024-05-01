### Cargo Commands ###

Verify Install:<br>
```cargo --version```

Generate a new Rust Project:<br>
```cargo new hello_world [--vcs git]```<br>
optional param overrides behavior and adds .gitignore in an existing Git repo

Generate a new Rust Project (without a Git repo):<br>
```cargo new hello_world ---vcs none```

Running ```cargo new``` also creates Cargo.toml
```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

...

[dependencies]
```

Compile then Execute:<br>
```cargo build```<br>
then<br>
```.\target\debug\hello_world.exe```

Compile and Execute in One:<br>
```cargo run```

Verify the code will compile:<br>
```cargo check```

Build to "release" instead of "debug":<br>
```cargo build --release```

Update dependencies:<br>
```cargo update```

Build documentation on all current dependencies and open in browser:<br>
```cargo doc --open```
