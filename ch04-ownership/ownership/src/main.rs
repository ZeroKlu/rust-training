fn main() {
    // Demonstrating variable scope
    {
        let s = "hello";
        println!("{s}"); // s is active, so this works
    }
    // s is out of scope, so this would produce a compile-time error
    // println!("{s}");

    println!("\n---\n");

    // Creating String variables
    {
        let s1 = "hello".to_string();
        let s2 = String::from("hello");
        println!("{s1} : {s2}");
    }

    println!("\n---\n");

    // Mutating string variables
    {
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{s}");
    }

    println!("\n---\n");

    // Moving a variable
    {
        let s1 = String::from("hello"); // s1 is valid (bound to the reference to the string)
        let s2 = s1; // s2 is valid, but s1 is no longer valid, because the string has moved to s2

        println!("{s2}"); // This is fine, but printing s1 (below) is an error
        // println!("{s1}");
    }

    println!("\n---\n");

    // Cloning a variable
    {
        let s1 = String::from("hello"); // s1 is valid (bound to the reference to the string)
        let s2 = s1.clone(); // s2 is valid, and s1 is still valid, because s2 is a clone

        println!("{s1} : {s2}"); // This is fine, since both s1 and s2 are valid
    }

    println!("\n---\n");

    // Stack-only copying
    {
        let x = 5;
        let y = x; // y and x are both valid, because y is a copy

        println!("{}", x + y)
    }

    println!("\n---\n");
}
