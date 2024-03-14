#[derive(Debug)]
enum Message {
    Quit,                        // No data: Just a variant
    Move {x: i32, y: i32},       // Two named fields
    Write(String),               // One String value
    ChangeColor(i32, i32, i32),  // Three integer values
}

impl Message {
    fn call(&self) {
        // TODO: Behaviors for different variants
        println!("{:?}", self);
    }
}

fn main() {
    let w = Message::Write(String::from("hello"));
    w.call();
}
