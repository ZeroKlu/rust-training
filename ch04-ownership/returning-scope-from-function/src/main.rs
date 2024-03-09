fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return to s1 (s1 is valid)

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back (s2 is not valid)
                                        // then, takes_and_gives_back moves its return to s3 (s3 is valid)

    println!("s1 = {s1} : s3 = {s3}");  // s1 and s3 are valid, so this is fine

    // s2 is invalid, so this won't work
    // println!("s2 = {s2}");

} // s1 and s3 go out of scope and are dropped. s2 was already moved, so nothing happens with it

fn gives_ownership() -> String {
    let some_string = String::from("yours");    // some_string comes into scope
    some_string                                 // some_string is returned and moved into s1
} // some_string goes out of scope but was already moved so nothing happens

fn takes_and_gives_back(a_string: String) -> String {  // a_string comes into scope
    a_string                                           // a_string is moved to s3
} // a_string goes out of scope but was already moved so nothing happens
