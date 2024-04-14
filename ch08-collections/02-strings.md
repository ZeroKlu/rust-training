## Storing UTF-8 Encoded Text with Strings ##

### What Is a String ###

In the Rust core language, there is only one string type:
the string slice ```str``` (usually use in its borrowed form
```&str```).

A string slice is a reference to a fixed of UTF-8 encoded
string data.

String literals (for example) are string slices.

---

The ```String``` type is defined in the Rust standard library
and, while still UTF-8 encoded, are growable, mutable, and
owned.

---

### Creating a New String ###

Because ```String``` is implemented as a vector of bytes,
many operations from ```Vec<T>``` carry over.

Create a new, empty String

```rust
// Create an instance of a String
let mut s = String::new();
```

Create and initialize a new String

```rust
let data = "Hello, World!";

// From a variable using `to_string`
let s = data.to_string();

// From a literal using `to_string`
let s = "Hello, World!".to_string();

// From a variable using `String::from`
let s = String::from(data);

// From a literal using `String::from`
let s = String::from("Hello, World!");
```

---

Since the ```String``` type is UTF-8 encoded, we aren't limited to ASCII. For example:

```rust
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
```

---

### Updating a String ###

We can grow a string by adding a string slice (using
```push_str```) or a single character (using ```push```)

```rust
let mut s1 = String::from("foo ");
s1.push_str("bar");

let mut s2 = String::from("foo ");
let s3 = "bar";
s2.push_str(s3);

let mut s4 = String::from("LO");
s4.push('L');
```

---

### Concatenation with the ```+``` Operator or the ```format!``` Macro ###

You can combine two strings using the concatenation operator
```+```

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

The code above follows a very specific signature:

```rust
// You can only add a `&str` to a `String`
fn add(self, s: &str) -> String { ... }
```

This is why ```&s2``` is passed as a reference and why
```s1``` becomes invalid.

---

But this gets unwieldy with multiple concatenations...

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```

... so, for scenarios like this, it is usually better to use
the ```format!``` macro:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");
```

---

### Indexing into Strings ###

You cannot index directly into a ```String```

```rust
// Note: This code will not compile
let s1 = String::from("hello");
let h = s1[0];
```

The attempt results in this error:

```
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0277]: the type `String` cannot be indexed by `{integer}`
 --> src/main.rs:3:13
  |
3 |     let h = s1[0];
  |             ^^^^^ `String` cannot be indexed by `{integer}`
  |
  = help: the trait `Index<{integer}>` is not implemented for `String`
  = help: the following other types implement trait `Index<Idx>`:
            <String as Index<RangeFrom<usize>>>
            <String as Index<RangeFull>>
            <String as Index<RangeInclusive<usize>>>
            <String as Index<RangeTo<usize>>>
            <String as Index<RangeToInclusive<usize>>>
            <String as Index<std::ops::Range<usize>>>

For more information about this error, try `rustc --explain E0277`.
error: could not compile `collections` due to previous error
```

But why? ...

---

The ```String``` type is a wrapper over ```Vec<u8>```, but 
it's not as simple as one byte (```u8```) per character.

---

For this code...

```rust
let hello = String::from("Hola");
println!("{}", hello.len());
```

... the output is:

```
4
```

That makes sense, given that we have 4 characters.

---

But for this code...

```rust
let hello = String::from("Здравствуйте");
println!("{}", hello.len());
```

... the output is:

```
24
```

We might have expected the output to be 12, since there are 12 characters, but some UTF-8 characters are two bytes long (
otherwise, we'd be limited to only 256 total characters in all
of the UTF-8 character set).

Because the Cyrillic alphabet is encoded as two bytes per
character, although we have 12 characters, the length is
24 bytes.

---

Since the indices of a ```Vec<u8>``` point to the individual 
bytes, and ```String``` is an implementation of ```Vec<u8>```,
an index might point to only part of a character's encoding, 
which is why we can't index directly into ```String```.

---

### Bytes and Scalar Values and Grapheme Clusters! Oh My! ###

There are three different ways to look at a ```String```'s 
content:

* Bytes
* Scalar Values
* Grapheme Clusters

For the word नमस्ते, written in the Devanagari script, we 
have:

* 18 Bytes
  ```
  [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
  ```
* Representing 6 3-byte characters (Unicode scalar values)
  ```
  ['न', 'म', 'स', '्', 'त', 'े']
  ```
* Which in turn represent four letters (grapheme clusters),
  because two of the scalar values ('्' & 'े') are
  combining diacritics (like accents) and form parts of the
  letters immediately preceding them
  ```
  ["न", "म", "स्", "ते"]
  ```

This further illustrates the restriction against indexing
directly into a ```String```

> Note: Accessing an item in a data structure should always
> take a constant time O(1) to complete. It would be 
> impossible to guarantee this if Rust permitted indexing into
> a ```String```

---

### Slicing Strings ###

Indexing into a string is often a bad idea because it's not 
clear what the return type of the string-indexing operation 
should be: a byte value, a character, a grapheme cluster, or 
a string slice. If you really need to use indices to create 
string slices, therefore, Rust asks you to be more specific.

Rather than indexing using ```[]``` with a single number, you 
can use ```[]``` with a range to create a string slice 
containing particular bytes:

```rust
let hello = "Здравствуйте";
let s = &hello[0..4];
```

Here, ```s``` will be a ```&str``` that contains the first 4 
bytes of the string. Earlier, we mentioned that each of these 
characters was 2 bytes, which means ```s``` will be ```Зд```

---

Rust would panic if we tried to obtain only part of a
character's bytes.

```rust
// This code will panic
let hello = "Здравствуйте";
let s = &hello[0..1];
```

```
thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/main.rs:4:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Because of this, string slices should be used with caution to 
avoid crashing the program.

---

### Methods for Iterating Over Strings ###

The best way to operate on pieces of strings is to be 
explicit about whether you want characters or bytes.

For individual Unicode scalar values, use the ```chars``` 
method.

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

Output...

```
З
д
```

---

Or, you can obtain the raw bytes using the ```bytes```
method.

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

Output...

```
208
151
208
180
```

---

The standard library does not provide a mechanism for
obtaining grapheme clusters, but there are existing crates that handle this complexity.

---

### Strings Are Not So Simple ###

Rust intentionally exposes more complexity to the programmer 
when handling ```String```s. The result is better error
prevention but more difficulty when programming.

---
