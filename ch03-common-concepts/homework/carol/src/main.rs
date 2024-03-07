fn main() {
    for n in 1..13 {
        get_verse(n);
    }
}

fn get_verse(n: i32) {
    let verses = [
        "partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "golden rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming"
    ];
    println!("On the {} day of Christmas, my true love gave to me...", ordinal(n));
    for day in (1..n + 1).rev() {
        let prefix = if day == 1 && n != 1 {"And "} else {""};
        let suffix = if day == 1 {".\n"} else {","};
        println!("{}{} {}{}", prefix, num_to_name(day), verses[(day - 1) as usize], suffix);
    }
}

fn ordinal(n: i32) -> String {
    let mut suffix = "th";
    if n == 1 {
        suffix = "st";
    }
    if n == 2 {
        suffix = "nd";
    }
    if n == 3 {
        suffix = "rd";
    }
    return format!("{n}{suffix}");
}

fn num_to_name(n: i32) -> &'static str {
    let numbers: [&str; 12] = ["a", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve"];
    if n < 1 || n > 12 {
        return "";
    }
    return numbers[(n - 1) as usize];
}
