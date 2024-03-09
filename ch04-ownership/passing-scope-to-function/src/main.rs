fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s);            // The value for s moves into the function
                                   // Since the value has moved, s is no longer valid
    
    let x = 5;                     // x comes into scope

    makes_copy(x);                 // Since x is i32, it is copied, not moved, so it remains valid
} // s and x both go out of scope, but s doesn't need to be dropped, because it already moved

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");            // some_string is valid, so I can use it here
} // some_string falls out of scope - drop is called, since it is owned by the function (memory freed)

fn makes_copy(some_integer: i32) {        // some_integer comes into scope
    println!("{some_integer}");            // some_integer is valid, so I can use it here
} // some_integer falls out of scope - nothing special happens, since it didn't require allocation
