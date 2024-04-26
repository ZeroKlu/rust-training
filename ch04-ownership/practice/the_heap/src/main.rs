// Beware! This example is a lie, as it assumes ownership doesn't exist

fn main() {
    let a = [0; 1_000];
    // Stack: main[a|ptr*]
    // Heap: [*0|0|0|0|0|...|0]
    // *Note: This is all lies, since arrays are allocated on the stack, not the heap,
    //        but we haven't learned vectors yet, and this is a worthwhile illustration

    let b = a;
    // Stack: main[a|/],[b|ptr*]
    // Heap: [*0|0|0|0|0|...|0] (only one box here - now owned by `b`)

    println!("{}", b[0]);
    // Stack & Heap unchanged

    // let x = Box::new([0; 100]);
    // Stack: main[x|ptr*]
    // Heap: [*0|0|0|0|0|...|0]

    // free(x); // Note this function doesn't exist

    // This would be undefined behavior, because `x` is accessed after it is freed
    // assert!(b[0] == 0);

    let a_num = 4;
    // Stack: main[a_num|4] 

    make_and_drop();
    // Stack: main[a_num|4] 

    println!("{a_num}");
}

fn make_and_drop() {
    let a_box = Box::new(5);
    // Stack: main[a_num|4]     make_and_drop[a_box|ptr*]
    // Heap [*5]
    
    println!("{a_box}");

    // Without ownership, this would be undefined behavior, because both
    // a_box and b_box would be deallocating the same heap memory when the
    // function ends.
    let b_box = a_box;
    
    println!("{b_box}");

    // With ownership, since `b_box` now owns the heap memory, this would
    //   be undefined behavior    
    // println!("{a_box}");
}
