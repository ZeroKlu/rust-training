## Bringing Paths into Scope with the use Keyword ##

Using lengthy paths can be inconvenient, so Rust includes the
```use``` keyword in order to shorten the paths that you call
in your code.

Consider this example:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; // Bring crate into scope

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // Call just the module
}
```

---

The ```use``` keyword only creates the shortcut for the scope in which it occurs.

```rust
// This code will not compile
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    // In a different module, the `use` is not in scope
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```

### Creating Idiomatic ```use``` Paths ###

We could have done this:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Here, the use shortcut includes the function
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
```

But this isn't idiomatic, as specifying the parent of the 
function makes it clear that the function is not in the
current scope.

---

However, when bringing a struct or enum into scope, it is
idiomatic to specify the full path:

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

---

... except if we are bringing multiple items with the same
name into scope:

Here, we're bringing two ```Result``` types into scope from
different modules.

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

This would not compile:

```rust
// Cannot compile due to name conflict
use std::fmt::Result;
use std::io::Result;

fn function1() -> Result {
    // --snip--
}

fn function2() -> Result<()> {
    // --snip--
}
```

---

### Providing New Names with the ```as``` Keyword ###

Alternatively, we can use the ```as``` keyword to alias one
of the types and eliminate the name conflict:

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

---

### Re-Exporting Names with ```pub use``` ###

We can allow the code to treat an item brought into scope as
though it was defined in the current scope, we can make it
public using the ```pub``` keyword.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

---

### Using External Packages ###

Using an external package requires two steps:

First, we must add the package as a dependency in Cargo.toml

```toml
rand = "0.8.5"
```

Then we must bring it into scope with ```use```

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

Applies to any package from [crates.io](https://crates.io/)

The exception is the ```std``` library. Although it is an
external crate, it is shipped with Rust, so it does not need 
to be added to Cargo.toml

---

### Using Nested Paths to Clean Up Large ```use``` Lists ###

When using multiple items from the same crate or module, the
```use``` list can get lengthy.

```rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
```

We can nest paths as follows to make these more concise

```rust
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

---

If the nested path includes the parent of another included item, like this:

```rust
use std::io;
use std::io::Write;
```

... we can use the ```self``` keyword for the parent

```rust
use std::io::{self, Write};
```

---

### The Glob Operator ###

If we want to bring all public items from a path into scope,
we can use the glob operator ```*```

```rust
use std::collections::*;
```
