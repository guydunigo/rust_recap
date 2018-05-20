// ------------------------------
// Enums
pub fn run() {
    enum IpAddrKind {
        V4,
        V6,
    }
    let ip = IpAddrKind::V6;

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    impl IpAddr {
        fn call(&self) -> &str {
            "salut"
        }
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    // The Option enum is like Haskell's Maybe
    // enum Option<T> {
    //  Some(T),
    //  None,
    // }

    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // ...
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky Penny");
                1
            }
            // Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(UsState::Alabama) => 12,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
            _ => 123,
        }
    }

    let i3 = Some(3);
    if let Some(3) = i3 {
        println!("three");
    } else {
        println!("not three");
    }
    // Same as :
    match i3 {
        Some(3) => println!("three"),
        _ => println!("not  three"),
    }
}
