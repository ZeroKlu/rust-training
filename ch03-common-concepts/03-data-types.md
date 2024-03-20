## Data Types ##

Rust is *statically-typed*, which means that all variable data types must be known at compile time.

---

### Scalar Data Types ###

Scalar data types represent a single value and come in four flavors:

* Integer
* Floating-Point
* Boolean
* Character

---

#### Integer Types ####

Where *n* is the number of bits in the integer type...
* Signed type values range from -(2ⁿ⁻¹) to 2ⁿ⁻¹ - 1
* Unsigned type values range from 0 to 2ⁿ - 1

Several integer types are available (see table below):

|Length|Similar Type|Signed|Unsigned|
|-|-|-|-|
|8-bit|byte|```i8```|```u8```|
|16-bit|short|```i16```|```u16```|
|32-bit|int|```i32```|```u32```|
|64-bit|long|```i64```|```u64```|
|128-bit|-|```i128```|```u128```|
|arch|architecture-dependent|```isize```|```usize```|
|||||

Integers support use of an underscore (a la Python) as a separator.
```rust
let a_million = 1_000_000;
```

Integer literals can be expressed as:

|Number Literals|Example|
|-|-|
|Decimal|```255```|
|Hexadecimal|```0xff```|
|Octal|```0o377```|
|Binary|```0b1111_1111```|
|Byte (u8 only)|```b'\*'``` (* = any ASCII char)|
|||

---

#### Floating-Point Numbers ####

There are two floating-point primitives in Rust: f32 and f64.

The default is f64 (on modern PCs, the speed difference is minimal between
single- and double-precision floats).

Rust floating-point types adhere to the
[IEEE-754](https://en.wikipedia.org/wiki/IEEE_754) standard

---

#### Boolean ####

The Boolean (bool) type is one byte in size and has two possible values:
```true``` and ```false```.

```rust
fn main() {
    // implicit bool
    let t = true;

    // explicit bool
    let f: bool = false;
}
```

---

#### Character ####

The character (char) type is four bytes in size and can contain any single
Unicode character.

In Rust:

* Characters are expressed in single-quotes
* Strings are expressed in double-quotes

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // Explicit type annotation
    let smiley = '☺';
    let emoji = '🙂';
    println!("{c} {z} {smiley} {emoji}");
}

// Output in pwsh: "z ℤ ☺ 🙂"
```

---

### Compound Data Types ###

Compound data types represent collections of values and come in two flavors:

* tuple
* array

---

#### Tuples ####

* Tuples are collections of values that can be of different data types
* Tuples are fixed-size (can neither grow nor shrink after declaration)

```rust
fn main() {
    // Type annotations (as below) are optional
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

* We can access tuple values using dot-index notation

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
}
```

* A tuple can unpack to multiple variables (*destructing*)

```rust
fn main() {
    let tup: = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("y = {y}");
}
```

* A tuple without any values ```()``` is called a *unit* and is the 
  default return from functions that do not explicitly return a value

---

#### Arrays ####

* Arrays are collections of values that must be of the same data type
* Arrays are fixed-size (can neither grow nor shrink after declaration)
    * Although not covered here, the *vector* data type is like a flexible-sized array
    * Arrays are better when the size is known and doesn't change
* Arrays are stored on the *stack* as opposed to the *heap* and are correspondingly fast

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];

    let months = [
        "January", "February", "March", "April",
        "May", "June", "July", "August",
        "September", "October", "November", "December"
    ];
}
```

* You can specify an array's data type explicitly.

```rust
fn main() {
    let nums = [i32; 5] = [1, 2, 3, 4, 5];
}
```

* You access array elements by index (0-based)

```rust
fn main() {
    let nums = [i32; 5] = [1, 2, 3, 4, 5];
    let three = nums[2];
}
```

* You can initialize an array with the same value at every index:

```rust
fn main() {
    // let arr = [true, true, true, true, true];
    let arr = [true; 5];
}
```

---

Attempting to access an array element outside the range of the
available indices will result in a runtime error (*panic*).

For example, if the user enters a number other than 0 through 4
in this code, we receive an 'index out of bounds' panic.

```rust
use std::io;

fn main() {
    let arr = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    lut mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number!");

    // Panic occurs here if index is out of range
    let element = arr[index];

    println!("The value at index {index} is: {element}");
}
```

---
