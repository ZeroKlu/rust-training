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

    // Note: Both arms must evaluate to the same data type
    let result = if condition {"Yes"} else {"No"};
}
```

---

### Loops ###

Loops allow repetitions of a code block and come in three flavors:

* loop
* while
* for

#### ```loop``` ####

The ```loop``` repeats until it is explicitly told to exit using
the ```break``` statement.

Note: ```break``` can specify the ```loop```'s return value

```rust
fn main() {
    let max = 10;
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter >= max {
            // End the loop
            break counter * 2;
        }
    };
    println!("Result: {result}"); // -> Result: 20
}
```

We can skip to the next iteration using ```continue```

```rust
fn main() {
    let max = 10;
    let mut counter = 0;
    let mut sum = 0;
    loop {
        counter += 1;
        if counter >= max {
            // End the loop
            break;
        }
        if counter % 2 == 0 {
            // Skip even numbers
            continue;
        }
        sum += counter;
        println!("{counter}");
    };
    println!("Sum: {sum}");
}
```

Rust supports loop labels to disambiguate multiple loops

```rust
fn main() {
    let mut count = 0;
    // We're labeling our outer loop
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // End the inner loop
                break;
            }
            if count == 2 {
                // Explicitly end the outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
```

#### ```while``` ####

A while loop executes as long as a condition is true

```rust
fn main() {
    let mut num = 3;
    while num != 0 {
        println!("{num}");
        num -= 1;
    }
    println!("LIFTOFF!!!");
}
```

Although you could create a ```while``` loop as shown below, in
Rust, the ```while true {}``` concept is equivalent to
```loop```

```rust
fn main() {
    let mut num = 0;

    // This works...
    // while true {
    //     if num == 5 {
    //         break;
    //     }
    //     num += 1;
    //     println!("{num}");
    // }

    // ... but this is equivalent and preferred
    loop {
        if num == 5 {
            break;
        }
        num += 1;
        println!("{num}");
    }
}
```

#### ```for``` ####

A ```for``` loop is used to iterate across a collection and is the
same as the *foreach* loop implemented in other languages like C#.

```rust
fn main() {
    let list = [1, 2, 3, 4, 5];
    for elem in list {
        println!("{elem}");
    }
}
```

You *could* implement the same loop using ```while```, but why *would* you?

```rust
fn main() {
    let list = [1, 2, 3, 4, 5];
    // Equivalent output to the for loop above, but more verbose
    //   and a little wasteful of memory and CPU
    let mut index = 0;
    while index < list.len() {
        println!("{}", list[index]);
        index += 1;
    }
}
```

A ```for``` loop can also elegantly replace a counter-style
```while``` loop by using a range in the form ```(min..max+1)```

You can (optionally) reverse the range with the ```rev()```
function.

```rust
fn main() {
    for n in (1..4).rev() {
        println!("{n}");
    }
    println!("LIFTOFF!!!");
}
```

---