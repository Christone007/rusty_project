fn main() {

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let m1 = Message::Write(String::from("This is a message"));

    println!("{:#?}", m1);
}
