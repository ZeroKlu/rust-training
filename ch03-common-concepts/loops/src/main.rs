fn main() {
    let max = 10;
    let mut counter = 0;
    // A simple loop
    let result = loop {
        counter += 1;
        if counter >= max {
            // End the loop and return specified value
            break counter * 2;
        }
        println!("{counter}");
    };
    println!("Result: {result}");

    println!("\n---\n");

    counter = 0;
    let mut sum = 0;
    // A loop with continue
    loop {
        counter += 1;
        if counter >= max {
            // End the loop
            break;
        }
        if counter % 2 == 0 {
            // Skip even numbers
            continue;
        }
        sum += counter;
        println!("{counter}");
    };
    println!("Sum: {sum}");

    println!("\n---\n");

    // Loop labels
    let mut count = 0;
    // We're labeling our outer loop
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // End the inner loop
                break;
            }
            if count == 2 {
                // Explicitly end the outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    println!("\n---\n");

    let mut num = 3;
    while num != 0 {
        println!("{num}");
        num -= 1;
    }
    println!("LIFTOFF!!!");

    println!("\n---\n");

    // This works...
    // while true {
    //     if num == 5 {
    //         break;
    //     }
    //     num += 1;
    //     println!("{num}");
    // }

    // ... but this is equivalent and preferred
    loop {
        if num == 5 {
            break;
        }
        num += 1;
        println!("{num}");
    }

    println!("\n---\n");

    // We can iterate a collection with while...
    let list = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < list.len() {
        println!("{}", list[index]);
        index += 1;
    }

    println!("\n---\n");

    // ... but it's more straightforward to use for
    for elem in list {
        println!("{elem}");
    }

    println!("\n---\n");

    // Using a range with the optional rev() *reversed* function
    for n in (1..4).rev() {
        println!("{n}");
    }
    println!("LIFTOFF!!!");

    println!("\n---\n");
}
