## Variables & Mutability ##

### Mutability ###

Rust favors immutability, and unless specifically declared
otherwise, Rust variables are immutable.

```rust
let x = 1;
// x is immutable, so we cannot assign a new value
// x = 2;

let mut y = 1;
// We declared y as mutable, so we can assign a new value
y = 2;
```

---

### Constants ###

Constants are always immutable and must be declared with a data
type and a value

It's conventional to name constants in UPPER_SNAKE_CASE

```rust
const SECONDS_PER_HOUR: u32 = 60 * 60;
```

---

### Shadowing ###

Shadowing allows the reuse of a variable name, either in the same
scope (to replace an existing immutable variable) or a separate one
(to define the variable for use within both scopes).

```rust
let num = 5;

// Note: With let, we can replace the immutable variable
// Techincally, this is also a shadow, but since it's in the same
//   scope, it is the only one visible to the compiler
let num = num + 1;

{
    // Within inner scope, this num shadows the existing num
    let num = num * 2;
    println!("Inner num = {num}") // 12
}

// After exiting the scope, the original variable is unshdowed
println!("Outer num = {num}") // 6

// We can shadow with a different data type for example,
//   provided we only need the integer value later in the program
let spaces = "   ";
let spaces = spaces.len();
```

---

