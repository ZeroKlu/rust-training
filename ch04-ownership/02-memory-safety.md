## Memory Safety ##

### Safety is the Absence of Undefined Behavior ###

Consider this safe program:

```rust
fn read(y: bool) {
    if y {
        println!("Condition is true.");
    }
}

fn main() {
    let x = true;
    read(x);
}
```

In x86 assembly, ```main()``` becomes:

```asm
main:
    ; ...
    mov     edi, 1
    call    read
    ; ...
```

... which performs two steps:

* Move the value ```1``` into the ```edi``` register
* Call the ```read``` function, which expects its argument to
  be the value in the ```edi``` register

---

We could make the code unsafe by moving the read to before the
variable is defined, like this:

```rust
// Note: This will not compile!
fn read(y: bool) {
    if y {
        println!("Condition is true.");
    }
}

fn main() {
    read(x); // We're trying to access x before initialization
    let x = true;
}
```

In x86 assembly, ```main()``` now becomes:

```asm
main:
    ; ...
    call    read
    mov     edi, 1      ; nov is after call
    ; ...
```

This has become unsafe. The call to ```read``` is expecting its
argument to be a boolean, but we haven't moved a value into the
```edi``` register yet.

The behavior at this point is ***undefined***. There is no way to
predict what data is stored in the register, so the result of
the ```read``` could be:

* Execution completes without an error, but the results are
  unreliable
* Execution crashes with something like a segmentation fault
* A bad actor has taken advantage of some other buffer
  overflow to insert malicious code into the register, and it is
  now executed.

Rust prevents this by not compiling with unsafe code like this
example.

---

### Advantages of Compile-Time Checking ###

1. The software has improved ***reliability***, because the 
   compiler has already caught any unsafe scenarios.
2. The software has improved ***performance***, because we've
   eliminated the majority of runtime checks.

---

### Ownership as a Discipline for Memory Safety ###

Ownership is used to prevent many different types of undefined
behavior, but the book focuses on operations on memory and on
the use of ownership as a discipline for safely handling
interactions with it.

---

### Variables Live in the Stack ###