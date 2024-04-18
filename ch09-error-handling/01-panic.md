## Unrecoverable Errors with ```panic!``` ##

There are two ways to trigger a panic in Rust:

1. Take an action that natively causes Rust to panic (out of 
   range index in an array, e.g.)
2. Explicitly call the ```panic!``` macro.

---

When a panic occurs, you have two choices:

1. Unwind (clean up) the stack (default behavior)<br>or...
2. Immediately abort without unwinding the stack

If you choose option 2, the OS must the clean up the memory that was in-use.

To use the abort mode for panic, update the Cargo.toml, 
adding ```panic = 'abort'``` to the appropriate profile(s).

e.g.:

```toml
[profile.release]
panic = 'abort'
```

---

### Calling ```panic!``` ###

When you call the ```panic!``` macro (as in this example)...

```rust
fn main() {
    // The `panic!` macro accepts a string argument that gets
    //   included in the error message
    panic!("crash and burn");
}
```

... the program halts and returns this error:

```
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

---

### Using a panic! Backtrace ###

When a library calls the panic due to a bug in our code, like 
in this example...

```rust
fn main() {
    let v = vec![1, 2, 3];

    // Code will panic because this index is out-of-range
    v[99];
}
```

... we still receive an error message

```
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

But we can get more information by adding a backtrace.

We do this by executing the following command on Windows:

```pwsh
$env:RUST_BACKTRACE=1; cargo run
```

... or, on Linux:

```bash
RUST_BACKTRACE=1 cargo run
```

The result is a more thorough trace of the error:

```
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src\main.rs:6:5
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library\std\src\panicking.rs:575
   1: core::panicking::panic_fmt
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library\core\src\panicking.rs:64
   2: core::panicking::panic_bounds_check
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library\core\src\panicking.rs:147
   3: core::slice::index::impl$2::index<i32>
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483\library\core\src\slice\index.rs:260
   4: core::slice::index::impl$0::index
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483\library\core\src\slice\index.rs:18
   5: alloc::vec::impl$15::index<i32,usize,alloc::alloc::Global>
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483\library\alloc\src\vec\mod.rs:2727
   6: simple_panic::main
             at .\src\main.rs:6
   7: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483\library\core\src\ops\function.rs:507
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

And we can expand the returned information even further by 
setting ```RUST_BACKTRACE=full```

Note: Since this is setting an environment variable, you'll 
have to use ```RUST_BACKTRACE=0``` to turn off the backtrace
for ```cargo run```

---
