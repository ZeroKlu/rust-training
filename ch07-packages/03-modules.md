## Defining Modules to Control Scope and Privacy ##

### Grouping Related Code in Modules ###

Modules let us organize code within a crate for readability 
and easy reuse. Modules also allow us to control the privacy 
of items, because code within a module is private by default.

As an example, let's write a library crate that provides the 
functionality of a restaurant.

In the restaurant industry, some parts of a restaurant are 
referred to as front of house and others as back of house. 
Front of house is where customers are; this encompasses where 
the hosts seat customers, servers take orders and payment, and bartenders make drinks. Back of house is where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative work.

To structure our crate in this way, we can organize its functions into nested modules.

Module Syntax:
```rust
mod module_name {
    // Functions and sub-modules go here
}
```

1. Create a new library named restaurant by running<br>
   ```cargo new restaurant --lib;```
2. enter the code below into src/lib.rs to define some 
   modules and function signatures.

*src/lib.rs*

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

The library crate module tree (defined in *src/lib.rs*) now 
looks like this:

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

