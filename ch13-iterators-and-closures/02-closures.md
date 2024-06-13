## Closures: Anonymous Functions that Capture Their Environment ##

The main feature of a closure is that it captures its environment at the 
time it is defined. It can then be used later, even after that environment
scope is gone.

---

### Capturing the Environment with Closures ###

Scenario:

*  Every so often, our t-shirt company gives away an exclusive, 
   limited-edition shirt to someone on our mailing list as a promotion. 
*  People on the mailing list can optionally add their favorite color to  
   their profile.
*  If the person chosen for a free shirt has their favorite color set,  
   they get that color shirt.
*  If the person hasn't specified a favorite color, they get whatever  
   color the company currently has the most of.

---

Example Code:

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // Passing closure `|| self.most_stocked()`
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```

---

In ```giveaway()```, we call ```user_preference.unwrap_or_else()```,
which evaluates the ```Some``` | ```None``` enum.

This function either provides the value in ```Some``` or the result
of the expression passed as its argument.

So, with ```user_pref1 = Some(ShirtColor::Red)``` this unwraps
```ShirtColor::Red```

However, with ```let user_pref2 = None;``` there is nothing to unwrap.
In ```giveaway()```, we call ```user_preference.unwrap_or_else()```.
So, the ```unwrap_or_else()``` function looks to its argument, which is
a closure without any parameters ```|| self.most_stocked()```

> Note: If an argument was passed to the closure, it would go between the
> two vertical pipe characters

---

So, when we execute this code with ```cargo run```, we see the
following results (as expected):

```
The user with preference Some(Red) gets Red
The user with preference None gets Blue
```

---

One interesting aspect here is that we've passed a closure that calls
```self.most_stocked()``` on the current ```Inventory``` instance. The 
standard library didn't need to know anything about the ```Inventory``` or 
```ShirtColor``` types we defined, or the logic we want to use in this 
scenario. The closure captures an immutable reference to the
```self Inventory``` instance and passes it with the code we specify to 
the ```unwrap_or_else``` method. Functions, on the other hand, are not 
able to capture their environment in this way.

---

### Closure Type Inference and Annotation ###

Closures typically do not require type annotations because they are not
exposed and are instead stored in variables.

You can include type annotations, even when not required.

```rust
let expensive_closure = |num: u32 | -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

All of the below are valid:

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  
```

---

Even though types are inferred in closures, once a closure is used
with a specific type, it cannot then be used with another type.

```rust
// This code will not compile
fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5); // Closure has already inferred String
}
```

```
error[E0308]: mismatched types
 --> src\main.rs:5:29
  |
5 |     let n = example_closure(5);
  |             --------------- ^- help: try using a conversion method: `.to_string()`
  |             |               |
  |             |               expected `String`, found integer
  |             arguments to this function are incorrect
  |
note: expected because the closure was earlier called with an argument of type `String`
 --> src\main.rs:4:29
  |
4 |     let s = example_closure(String::from("hello"));
  |             --------------- ^^^^^^^^^^^^^^^^^^^^^ expected because this argument is of type `String`
  |             |
  |             in this closure call
note: closure parameter defined here
 --> src\main.rs:2:28
  |
2 |     let example_closure = |x| x;
  |
```

---

