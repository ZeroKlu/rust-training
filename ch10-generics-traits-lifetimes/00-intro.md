## Generic Types, Traits, and Lifetimes ##

**Generics** are abstract stand-ins for concrete types and 
can be passed to functions in lieu of them.

**Traits** define behavior generically.

**Lifetimes** provide information to the compiler about how 
references relate to each other.

---

### Removing Duplication by Extracting a Function ###

Generics allow us to replace specific types with a 
placeholder that represents multiple types to remove code 
duplication.

First we'll look at a non-generic way to remove duplication by
*extracting* a function.

This code finds the largest number in a list...

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

---

But if we needed to find the largest number in each of
multiple lists, we would end up duplicating code like this...

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

---

Extracting the code that finds the largest number into a
separate function allows us to accomplish the same task 
without duplication.

```rust
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
```

---
