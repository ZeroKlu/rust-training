## Traits: Defining Shared Behavior ##

A *trait* defines functionality a particular type has and can 
share with other types. We can use traits to define shared 
behavior in an abstract way. We can use *trait bounds* to 
specify that a generic type can be any type that has certain 
behavior.

> The book points out the similarity between traits and what
> other languages call *interfaces*.

---

### Defining a Trait ###

Imagine the scenario where you have two different structs, 
each of which holds some text content (of varying size and 
type). If we need a method to summarize these contents
across multiple types, we could define it in a trait:

To follow along, create a new project using:

```
cargo new aggregator
```

Create a file ```lib.rs``` in the /src folder and add this 
code.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

This indicates that any type possessing the Summary trait must
implement a ```summarize()``` method.

---

### Implementing a Trait on a Type ###

Having defined the trait, it must now be implemented on the
type(s) that will use it.

In src/lib.rs, add the following: 

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

---

Now that we have implemented the trait, we can add the
following to src/main.rs:

```rust
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```

When we run the code, we get this output:

```
1 new tweet: horse_ebooks: of course, as you probably already know, people
```

---

Book Note: To implement a trait on a type, either the trait 
or the type must be local to the crate.

* Implement the standard library trait ```Display``` on
  ```Tweet```?
    * YES: Because ```Tweet``` is local to our crate
* Implement ```Summary``` on ```Vec<T>```?
    * YES: Because ```Summary``` is local to our crate
* Implement ```Display``` on ```Vec<T>```?
    * NO: Neither is in the ```aggregator``` crate.

This is a result of the so-called *orphan rule*, which 
prevents two crates from breaking each other's code (for
example by both implementing traits with the same name).

---

### Default Implementations ###

You can specify behavior in the trait definition itself
rather than requiring it to be implemented in the individual
types.

```rust
// in lib.rs
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

```rust
// in main.rs
fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}
```

Output:

```
New article available! (Read more...)
```

Note: If the method is implemented in a type, the type's
implementation will automatically override the default.

---

You can mix and match. For example, a method with a default
implementation can call another method in the trait that
doesn't have a default.

In lib.rs
```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    // Now we only have to implement `summarize_author`
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

In main.rs
```rust
fn main() {
        let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```

Output:
```
1 new tweet: (Read more from @horse_ebooks...)
```

---

### Traits as Parameters ###

You can define a function to accept data types that implement
a trait as parameters.

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    // --snip--

    notify(&tweet);
}
```

---

### Trait Bound Syntax ###

The above ```impl Trait``` syntax is syntactic sugar for a
longer form known as a *trait bound*.

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

This is useful when you have multiple arguments requiring a
trait, like this:

```rust
// Using `impl Trait`
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
```

```rust
// Using trait bound
pub fn notify<T: Summary>(item1: &T, item2: &T) {}
```

---

### Specifying Multiple Trait Bounds with the ```+``` Syntax ###

We can require multiple traits using ```+``` like this:

```rust
// Using `impl Trait`
pub fn notify(item: &(impl Summary + Display)) {}
```

```rust
// Using trait bound
pub fn notify<T: Summary + Display>(item: &T) {}
```

### Clearer Trait Bounds with ```where``` Clauses ###

Function signatures can become cluttered when there are 
multiple trait bounds.

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    // ...
}
```

We can make these more readable by using ```where``` clauses.

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}
```

---

### Returning Types that Implement Traits ###

We can also use the ```impl Trait``` syntax in the return 
position to return a value of some type that implements a 
trait.

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

---

Note: You can only specify a return type by trait if you are
returning a single type.

```rust
// Note: This code will not compile.
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

---

### Using Trait Bounds to Conditionally Implement Methods ###

In this example ```Pair<T>``` always implements the ```new```
function but only implements the ```cmp_display``` method if
its inner type ```T``` also implements the ```PartialOrd```
trait.

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

---

We can also conditionally implement a trait for any type that 
implements another trait.

Such *blanket traits* are used throughout the standard 
library. For example:

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```

This allows:

```rust
let s = 3.to_string();
```

---
