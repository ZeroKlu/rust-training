## Separating Modules into Different Files ##

Unlike the textbook examples, for larger projects, it is 
typical to separate modules into different files, both to
make code more reusable and to keep the project understandable
for developers.

To do this, we can remove the content of a module from
*lib.rs* (or from *main.rs* in a binary project) and place it
in a file with the same name as the module.

We leave the declaration of the module in *lib.rs*, like this:

In *lib.rs*
```rust
// This declaration now points to front_of_house.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

We don't separately declare the module in its file, as the
module only needs to be declared once in the module tree.

In *front_of_house.rs*:
```rust
// This file is now `mod front_of_house`
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

---

We can take this a step further and extract the hosting 
module from front_of_house.

In *front_of_house.rs*:
```rust
pub mod hosting;
```

We'll create a directory called *front_of_house* and create a file called *hosting.rs*

In *front_of_house/hosting.rs*
```rust
pub fn add_to_waitlist() {}
```

---

> ### Alternate File Paths ###
> 
> So far we've covered the most idiomatic file paths the Rust 
> compiler uses, but Rust also supports an older style of 
> file path. For a module named front_of_house declared in 
> the crate root, the compiler will look for the module's 
> code in:
> 
> * ***src/front_of_house.rs*** (what we covered)
> * ***src/front_of_house/mod.rs*** (older style, still 
> supported path)
> 
> For a module named hosting that is a submodule of 
> front_of_house, the compiler will look for the module's 
> code in:
> 
> * ***src/front_of_house/hosting.rs*** (what we covered)
> * ***src/front_of_house/hosting/mod.rs*** (older style, 
> still supported path)
> 
> If you use both styles for the same module, you'll get a 
> compiler error. Using a mix of both styles for different 
> modules in the same project is allowed, but might be 
> confusing for people navigating your project.
> 
> The main downside to the style that uses files named mod.rs 
> is that your project can end up with many files named
> mod.rs, which can get confusing when you have them open in 
> your editor at the same time.
>
> ---

---
