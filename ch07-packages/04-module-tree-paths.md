## Paths for Referring to an Item in the Module Tree ##

### Using Paths ###

Calling a function in a module requires a **path**

Paths come in two flavors:

* Absolute Path:
    * Full path starting from crate root
        * External Crate: Start with crate name
        * Current Crate: Start with literal: ```crate```
* Relative Path:
    * Starts from the current module and uses:
        * ```self``` (current module)
        * ```super``` (parent module)
        * an identifier in the current module

Recall the previous code sample:

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        // * SNIP *
    }
    // * SNIP *
}
```

If we want to call the ```add_to_waitlist``` function, we can
use either flavor of path:

```rust
// Note: This code will not compile (yet)
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

Because we declared the ```eat_at_restaurant``` function as
```pub``` (public), it cannot expose calls to private members
of the crate, so we receive this error if we try to build:

```
cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^ private module
  |
note: the module `hosting` is defined here
 --> src/lib.rs:2:5
  |
2 |     mod hosting {
  |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^ private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors
```

---

### Exposing Paths with the ```pub``` Keyword ###

We can modify the code to make the ```hosting``` module public

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
        // * SNIP *
    }
    // * SNIP *
}

// Note: This code will still not compile (yet)
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

... but that still results in a build error:

```
cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: function `add_to_waitlist` is private
 --> src/lib.rs:9:37
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                                     ^^^^^^^^^^^^^^^ private function
  |
note: the function `add_to_waitlist` is defined here
 --> src/lib.rs:3:9
  |
3 |         fn add_to_waitlist() {}
  |         ^^^^^^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:12:30
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src/lib.rs:3:9
   |
3  |         fn add_to_waitlist() {}
   |         ^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors
```

Even though ```hosting``` is now public, its contents aren't.

So we need to make the ```add_to_waitlist``` function public 
too.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        // * SNIP *
    }
    // * SNIP *
}

// Note: Now this code will compile
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

---

### Starting Relative Paths with ```super``` ###

For paths that begin in the parent module, we can start the a
relative path with the ```super``` keyword.

Consider this code sample (in *src/lib.rs*)

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // Relative path to function in parent module
        super::deliver_order();
    }

    fn cook_order() {}
}
```

The ```deliver_order``` function is not in the 
```back_of_house``` module, but in its parent, so when we call
that function inside ```fix_incorrect_order```, we use the
```super``` keyword to move up a parent level in the relative
path.

Since the parent is the crate, we could have used this
absolute path equivalently:

```rust
    // ...
    crate::deliver_order();
    // ...
```

---

### Making Structs and Enums Public ###

The ```pub``` keyword can also be used to expose a struct or
enum.

In this example, we add a ```Breakfast``` struct to the 
```back_of_house``` module. We make the ```toast``` member
public as well, to allow the caller to choose a type, but
leave the ```seasonal_fruit``` member private.

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // Without this function an external call
        //   could not instantiate Breakfast
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it
    // We're not allowed to see or modify the seasonal fruit 
    //   that comes with the meal

    // meal.seasonal_fruit = String::from("blueberries");
}
```

When we make an enum public, all of its variants are
automatically public, so we do not need to apply the 
```pub``` keyword to them.

```rust
mod back_of_house {
    // ** SNIP **
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // ** SNIP **
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

