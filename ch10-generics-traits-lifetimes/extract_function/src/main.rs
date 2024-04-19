// Example 1 - Finding the largest number in a list
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let mut largest = &number_list[0];

//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);
// }

// Example 2 - Finding the smallest number in two lists
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let mut largest = &number_list[0];

//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);

//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let mut largest = &number_list[0];

//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);
// }

// Example 3 - Extracting the function
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list1 = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list1);
    println!("The largest number is {}", result);

    let number_list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list2);
    println!("The largest number is {}", result);

    for l in vec![&number_list1, &number_list2] {
        let result = largest(l);
        println!("The largest number is {}", result);
    }
}