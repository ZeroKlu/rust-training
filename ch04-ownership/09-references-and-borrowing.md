## References ands Borrowing ##

The real answer to the problem of passing a variable without transferring
ownership is to use references.

### Definition ###

**Reference**:

* The reference is a pointer to the pointer held by the original
  variable.
* Syntax:
    * Indicated by an ampersand [```&```]
    * ```&variable_name``` for an argument being passed to a function
    * ```&data_type``` for a parameter being received by a function
* A reference is a like a pointer because
    * It contains the address where a value owned by another variable is 
      located on the heap.
* A reference differs from a pointer because:
    * It is guaranteed to contain a value for the life of the reference

Note: An asterisk [```*```] can be used as the dereference operator
(```*param_name```, e.g.)

---

### Passing a Reference to a Function ###

Rewriting the code where we returned multiple values as an ownership
workaround, we can pass the string by reference instead.

```rust
fn main() {
    let s1 = String::from("hello"); // s1 comes into scope
    // Passing the reference pointer (&s) does not transfer ownership
    let len = calculate_length(&s);
    println!("Length of '{s1}' is {len} characters."); // s1 is still valid
} // s1 goes out of scope and is dropped, because it is still valid

// Accepting a reference (&String) does not take ownership
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope but nothing is dropped here,
  //   because the function does not own s
```

In the example above...

**s** is bound to a variable like this, stored on the stack:
|name|value|explanation|
|-|-|-|
|```ptr```|address|stack memory location where the **s1** pointer is stored|
|||

**s1** remains bound to a value like this, stored on the stack:

|name|value|explanation|
|-|-|-|
|```ptr```|address|heap memory location of index 0 in the string value|
|```len```|5|length (in bytes) of the string value|
|```capacity```|5|maximum length (in bytes) that can be stored in the string|
||||

The pointer ```ptr``` points to the memory location (on the heap) where the string is stored as a collection of characters (like this).

|loc|index|value|
|-|-|-|
|```ptr```|0|h|
|```ptr + 1```|1|e|
|```ptr + 2```|2|l|
|```ptr + 3```|3|l|
|```ptr + 4```|4|o|
||||

This "pointer to a pointer" concept (called *borrowing*) enables working
with a value without taking ownership.

---

### Modifying a Borrowed Value ###

When we try to modify a value borrowed by a function, it results in an
error, because the borrowed value is not owned by the function.

So this code would fail..

```rust
fn main() {
    let s1 = String::from("hello");
    change(&s1);
    println!("{s1}");
}

fn change(s: &String) -> usize {
    // Error: `s` is a `&` reference, so the data it refers to cannot be
    //   borrowed as mutable
    s.push_str(", world");
}
```

But, just like we can make a variable mutable, we can also create a
mutable reference.

```rust
fn main() {
    // To allow changes in a borrowing function, we...
    let mut s1 = String::from("hello"); // 1. Make the variable mutable
    change(&mut s1); // 2. Declare the reference as mutable
    println!("{s1}");
}

// 3. Mark the parameter as mutable
fn change(s: &mut String) -> usize {
    // Now, this will work
    s.push_str(", world");
}
```

However, when you have a mutable reference, *you cannot have any other
references* to the original variable.

So this would produce an error...

```rust
let mut s = String::from("hello");

let ref1 = &mut s; // This is OK
let ref2 = &mut s; // This is an error (second mutable reference)
```

This is to prevent the scenario called a *data race*, which is like a
*race condition* and occurs when:

* Two or more pointers access the same data at the same time
* At least one of the pointers is used to modify (write) the data
* No mechanism is used to synchronize the data

Many other languages allow mutation anywhere, but that does not prevent
data races.

---
