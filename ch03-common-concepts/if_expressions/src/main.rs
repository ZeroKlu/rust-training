fn main() {
    let num = 3;

    if num % 2 == 0 {
        // Code in the if block executes only when the condition is true
        println!("{num} is even!");
    } else {
        // Code in the else block executes when the condition is false
        println!("{num} is odd!");
    }

    let t = true;
    // This is fine, because t is a bool
    if t {
        println!("Condition 'T' was true!")
    }

    // let n = 1;
    // This would result in an error, because n is an integer
    // if n {
    //     println!("Condition 'N' was true!")
    // }

    let age = 25;
    let mut price = 20;

    if age >= 65 {
        price = 15;
    } else if age <= 5 {
        price = 0;
    } else if age <= 18 {
        price = 10;
    }

    println!("Admission: ${price}");
    
    let condition = false;
    let result = if condition {"Yes"} else {"No"};
    println!("Result: {result}");
}
