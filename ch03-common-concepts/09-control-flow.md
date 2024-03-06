## Control Flow ##

Control flow refers to the ability to determine how code will execute
depending on whether or not some condition is true.

Control flow in Rust primarily takes two forms:

* *if* expressions
* loops

---

### *if* Expressions ###

An *if* expression branches the code based on the logical (Boolean)
value of a condition.

```rust
fn main() {
    let num = 3;

    if num % 2 == 0 {
        // Code in the if block executes only when the condition is true
        println!("{num} is even!");
    }
}
```

We can add an *else* branch for code to execute when the condition is
false

```rust
fn main() {
    let num = 3;

    if num % 2 == 0 {
        // Code in the if block executes only when the condition is true
        println!("{num} is even!");
    } else {
        // Code in the else block executes when the condition is false
        println!("{num} is odd!");
    }
}
```

Some languages support so-called "truthy" values. For example, in
Python, an integer with a non-zero value can be called like this:

```python
x = 10
if x:
    # do something
```

Rust does not support this (the condition must be a bool), so:

```rust
fn main() {
    let t = true;
    // This is fine, because t is a bool
    if t {
        println!("Condition t was true!")
    }

    let n = 1;
    // This will result in an error, because n is an integer
    if n {
        println!("Condition n was true!")
    }
}
```

We can add any number of *else if* branches to handle multiple
conditions

```rust
fn main() {
    let age = 25;
    let mut price = 20;

    if age >= 65 {
        // Executes if the condition is true
        price = 15;
    } else if age <= 5 {
        // Executes if the first condition was false,
        //   but this condition is true
        price = 0;
    } else if age <= 18 {
        // Executes if the all preceding conditions were false,
        //   but this condition is true
        price = 10;
    } else {
        // Executes if none of the conditions were true
    }

    println!("Admission: ${price}");
}
```

You can use an *if*/*else* expression in a 'let' statement.

```rust
fn main() {
    let condition = true;

    let result = if condition {"Yes"} else {"No"};
}
```

---

### Loops ###

---