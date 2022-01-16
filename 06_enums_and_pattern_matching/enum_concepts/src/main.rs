fn main() {
    //#1
    //Now IpAddrKind is custom data type.
    enum IpAddrKind {
        V4,
        V6,
    }

    //#2
    //instead of struct for address, we can just store value in enum
    //A advantage of enum is you can use different type for each variants.
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    //#3 another example
    //A key difference beteween struct and enum is
    //enum have variants of same type, struct has each types of structs.
    enum Message {
        _Quit,
        _Move { x: i32, y: i32 },
        Write(String),
        _ChangeColor(i32, i32, i32),
    }
    //you can also define method for enum
    impl Message {
        fn call(&self) {
            println!("did you call me?")
        }
    }

    //#4 Rust does not have nulls ,
    //but it has an enum that can encode the concept of a value being present or absent
    //This enum is 'Option<T>' (defined by std)
    //<T> syntax is generic type parameter in Chapter 10
    //For now, <T> means the 'Some' variant can hold one piece of data of any type.
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    //#1
    let _four = IpAddrKind::V4; //instances of V4, type is IpAddrKind
    let _six = IpAddrKind::V6; //instances of V6, type is IpAddrKind

    //#2
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    //#3
    let m = Message::Write(String::from("hello"));
    m.call();

    //#4
    let _some_number = Some(5); // -> now Option<T> is Option<i32> type

    //When we have a Some value, we know that a value is present and the value is held within the Some.
    //When we have a None value, in some sense, it means the same thing as null: we don’t have a valid value.
}
