## Generic Data Types ##

We use generics to create definitions for items like function 
signatures or structs, which we can then use with many 
different concrete data types.

---

### In Function Definitions ###

Consider the scenario where we need our largest number 
function to work for both ```i32``` and ```char```.

Without generics we would have to write two functions, one for
each data type.

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
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

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
```

These two functions differ only in the data type of the list
and the return.

---

We can declare this function generically like this:

```rust
fn largest<T>(list: &[T]) -> &T {...}
```

An initial implementation of this function might look like this:

```rust
// Note: This code will not compile
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

... but this results in the following error:

```
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src\main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
```

Because the generic type ```T``` in unknown, the compiler
cannot identify whether it possesses the ```PartialOrd```
trait, which would allow us to compare using ```>```.

If a generic ```T``` needs to be comparable, the function must
explicitly tell the compiler about this requirement.

And generics have no core functions: they cannot be printed,
cloned, or mutated. They can, however, be dropped.

---

### In Struct Definitions ###

You can also define a struct using a generic type, like this:

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

---

As currently defined, both ```x``` and ```y``` in the struct
must be of the same type, so this would be invalid:

```rust
struct Point<T> {
    x: T,
    y: T,
}

// Note: This code will not compile
fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```

And would result in this error:

```
error[E0308]: mismatched types
 --> src/main.rs:7:38
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integer, found floating-point number
```

---

But we can allow different types by specifying them as 
follows:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

---

### In Enum Definitions ###

We can also use generics in enum types.

For example, the built-in ```Option``` type is defined with
a generic type.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

And ```Result``` is generic over two types

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

---

### In Method Definitions ###

Like functions, methods implemented on a struct can also use
generics:

```rust
struct Point<T> {
    x: T,
    y: T,
}

// Note: `impl` must also specify the generic type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

---

We could also specify some methods to be type-specific:

```rust
// This method will only be available on Point<f32>
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

---

Method generic types do not necessarily have to be the same
as the type(s) defined on the struct.

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    // Types X2 and Y2 can differ from X1 and Y1
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

### Performance of Code Using Generics ###

Rust implements a system of *monomorphization* (the compiler
reduces the generic types to the concrete types used) to
prevent any performance degradation when using generics.

So, if we use the ```Option<T>``` type like this in our code:

```rust
fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

... the compiler identifies the types used and morphs the
code into this before compiling:

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

---
