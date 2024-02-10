fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            println!("kirekhar");
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // let _home = IpAddr::V4(String::from("127.0.0.1"));

    // let _loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
