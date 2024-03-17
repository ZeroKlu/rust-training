## Catch-All Patterns and the _ Placeholder ##

When there is only a need for special handling on some cases, we can specify a
default behavior for everything else.

In Rust, we can do this using the ```other``` case.<br>
Note: When using this case, it must cope last in the ```match``` options.

The book offers an example where, in a dice game
* A roll of 3 means you don't move, but you get a fancy hat
* A roll of 7 means you don't move, and you lose a fancy hat
* Any other roll means you move the rolled number of spaces

```rust
fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player( num_spaces: u8) {
    println!("{}", num_spaces);
}
```

---

When we don't need to use the value from the catch-all, we can substitute
```_``` for ```other```.

If we change the riles in the previous game such that on any roll other than
3 or 7, we do nothing, the code would look like this:

```rust
fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // Use the unit value to indicate we do nothing
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

---
