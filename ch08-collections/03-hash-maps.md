## Storing Keys with Associated Values in Hash Maps ##

A hash map (```HashMap<K, V>```) is a lot like a dictionary
in C# or Python and stores a mapping of keys of type
```K``` and values of type ```V```.

---

### Creating a New Hash Map ###

A common method for creating a new hash map is to use
```new``` and then use the ```insert``` method to add
key/value pairs.

```rust
// We do need to explicitly being HashMap into scope
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

Note: All keys must be of the same type as must all values

---

### Accessing Values in a Hash Map ###

The ```HashMap``` type exposes a ```get``` method, which we
use to access values by key.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

---

We can also iterate over all values with a loop

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{key}: {value}");
}
```

Note: HashMaps are not ordered, so the loop will return the
values in an arbitrary order.

---

### Hash Maps and Ownership ###

For types that implement the Copy trait, like ```i32```, the 
values are copied into the hash map. For owned values like 
```String```, the values will be moved and the hash map will 
be the owner of those values.

```rust
    use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// `field_name` and `field_value` are invalid at this point. 
//   try using them and see what compiler error you get!
```

---

### Updating a Hash Map ###

Each unique key in a ```HashMap``` can have only one value.

---

#### Overwriting a Value ####

So, even though this code inserts twice...

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```

... the output will have only one element

```
{"Blue": 25}
```

The second ```insert``` call overwrote the first value,
because that key can only exist once in the hash map.

---

#### Adding a Key and Value Only If a Key Isn’t Present ####

We can use the ```HashMap```'s ```entry``` API to check if
a key already exists. This returns an ```Entry``` enum,
which implements the ```or_insert``` method which returns
the a mutable reference to the existing ```Entry``` if there 
is one, or inserts the new value if not.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

// This WILL insert a new value
scores.entry(String::from("Yellow")).or_insert(50);
// This will NOT insert a new value
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```

Output:

```
{"Yellow": 50, "Blue": 10}
```

---

#### Updating a Value Based on the Old Value ####

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```

Output:

```
{"world": 2, "hello": 1, "wonderful": 1}
```

---

### Hashing Functions ###

By default, ```HashMap``` uses a hashing function called 
*SipHash* that can provide resistance to Denial of Service 
(DoS) attacks involving hash tables.

This is not the fastest hashing algorithm available, but the 
trade-off for better security that comes with the drop in 
performance is worth it.

If you profile your code and find that the default hash 
function is too slow for your purposes, you can switch to 
another function by specifying a different hasher. A *hasher* 
is a type that implements the ```BuildHasher``` trait.

---
