## Variable Interactions With Move ##

Allowing multiple variables to interact with the same data is one of the
main challenges in memory management.

There are several different ways this behaves in Rust.

* In this instance, with two variables of fixed and known size, both are
pushed onto the stack

    ```rust
    let x = 5; // bind the value 5 to x
    let y = x; // copy the value in x and bind the copy to y
    ```

* However, things get complicated when values are stored on the heap.

    ```rust
    let s1 = String::from("hello"); // bind the pointer to s1
    let s2 = s1; // copy the pointer (not the value) and bind to s2
    ```

In the example above, **s1** is bound to a value like this, stored on the stack:

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

**s2** is bound to a copy of the string reference value (containing the 
pointer), but the actual string is not copied. So both s1 and s2 are pointing to the same memory location (and therefore the same string value) on the heap.

Because of this, if both variables s1 & s2 were active, we would have an 
issue where when the two variables go out of scope, they would be trying to 
free the same heap memory twice (double-free error).

Instead, in Rust, only one variable can actively point to the value in 
memory, so:

```rust
let s1 = String::from("hello"); // s1 is valid
let s2 = s1; // s2 is valid, but s1 is no longer valid

println!("{s1}"); // This would cause an error, because s1 is not valid
```

So, Rust doesn't need to call ```drop``` on s1, only on the valid value s2,
and the double-free error is prevented (at the cost of not being able to use
s1 later in the code).

In Rust terms, the string that was originally owned by s1 has *moved* to s2.

**move**
* A Rust ```move``` is similar to a shallow copy in other languages (where a
  copy is made only of the reference, not the value).
* Where ```move``` differs is that only the new copy is active

Note: By implication, this means that Rust will never *automatically* make a deep copy of the original variable.

---

### Variables Cannot Be Used After Being Moved ###

If we imagine a scenario where Rust does not prevent accessing a
moved variable, we might see something like this:

```rust
// Note: This will not compile
fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first); // Variable is moved
    // Here, we attempt to access the moved variable 'first'
    println!("{full} was originally {first}"); // [L1]
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
```

<image src="../additional-files/images/diagram0401g.png"
       style="width:260px;" alt="Diagram 4.gf"
       title="Diagram 4.1g">

---

### Moved Heap Data Principle ###

If a variable ```x``` moves ownership of heap data to another
variable ```y```, then ```x``` cannot be used after the move.

---
