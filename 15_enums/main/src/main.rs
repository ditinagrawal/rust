fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    impl Message {
        fn call(&self) {
            println!("Message::call()");
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
