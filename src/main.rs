#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    // Variable mutable (otherwise constants) only with `mut`
    let mut x = 5;
    // Actual constants, must have a type defined
    const Y: u32 = 100_000;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);
    // Shadow variables
    let x = "a";

    // ------------------------------
    // Types

    let guess: i32 = "-42".parse().expect("Not a number!");
    println!("guess = {}", guess);
    // 98_222
    // 0xff
    // 0o77
    // 0b1111_0000
    // b'A'

    let guess: f32 /*f64*/ = 3.2;
    let guess: bool = false;
    let guess: char = 'z';

    // ------------------------------
    // Tuples

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring
    let (x, y, z) = tup;
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;

    let arr = [1, 2, 3, 4, 5];
    let x = arr[3];

    // ------------------------------
    // Functions, expressions (without ';', return a value), statements

    fn function_sample(x: i32) {
        println!("Another function. {}", x);
    }
    fn five() -> i32 {
        5 // no ';' because only expressions return a value
    }

    function_sample(x);

    let y = {
        let x = 3;
        x + 1 // expression that is returned from the block
    };

    // ------------------------------
    // Control flow

    if x < 5 {
        println!("< 5");
    }
    else {
        println!(">= 5");
    }

    loop {
        break;
    }

    let mut i = 0;
    while i != 3 {
        i = i+1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for element in (1..4).rev() {
        println!("the value is: {}", element);
    }

    // ------------------------------
    // Ownership, moving, borrowing, slices

    let mut s = String::from("hello");
    s.push_str(", world!");
    let s2 = s.clone(); // Deep copy
    let s2 = s; // Moved (can't use s anymore)

    fn take_ownership(s: String) {
        println!("{}", s);
    }

    take_ownership(s2); // The string goes to never return


    fn borrowing(s: &String) -> usize {
        s.len()
    }
    fn add_stuff(s: &mut String) {
        s.push_str(", hehe");
    }

    let mut s = String::from("hello");
    borrowing(&s);
    add_stuff(&mut s);

    let slice = &s[3..6];
    let slice = &s[..6];
    let slice = &s[2..];
    println!("{}, {}", slice, s);

    // &str is for slices so using it for parameters enables us to use &String and slices (&str)
    fn first_world(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let s = String::from("hello world");
    let word = first_world(&s);
    let word = first_world(&s[..]);
    // String literals are string slices (&str)
    let string_literal = "hello world";
    let world = first_world(string_literal);

    let x = [1, 2, 4, 5, 3];
    let slice = &x[1..3];

    // ------------------------------
    // Structs

    struct User {
        username: String,
        email: String,
        sing_in_count: u64,
        active: bool
    }

    fn build_user(email: String, username: String) -> User {
        User {
            email, // don't have to repeat "email: email"
            username,
            active: true,
            sing_in_count: 1,
        }
    }

    let mut user = User {
        email: String::from("s@s.s"),
        username: String::from("s"),
        active: true,
        sing_in_count: 1
    };
    user.email = String::from("a@a.a");

    let user = build_user(String::from("i@i.i"),String::from("i"));
    // Use values from another user to init the struct
    let user2 = User {
        email: String::from("2@2.2"),
        username: String::from("2"),
        ..user
    };

    // Tuple structs
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    // ------------------------------
    // Struct methods

    struct Rectangle {
        w : u32,
        h : u32,
    };

    impl Rectangle {
        fn area(&self) -> u32 {
            self.w * self.h
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            (self.w > other.w) && (self.h > other.h)
        }
        // associated function
        fn square(size: u32) -> Rectangle {
            Rectangle { w: size, h: size }
        }
    }

    let r = Rectangle { w: 3, h: 2 };
    let r2 = Rectangle::square(3);
    println!("R(3,2) area : {}", r.area());
    println!("R(1,2) in R(3,2) ? {}", r.can_hold(&r2));

    // ------------------------------
    // Enums

    enum IpAddrKind {
        V4,
        V6,
    }
    let ip = IpAddrKind::V6;

    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    impl IpAddr {
        fn call(&self) -> &str {
            "salut"
        }
    }

    let home = IpAddr::V4(127,0,0,1);

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
            },
            // Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(UsState::Alabama) => 12,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
            _ => 123
        }
    }

    let i3 = Some(3);
    if let Some(3) = i3 {
            println!("three");
    }

    // ------------------------------
}

// ------------------------------

fn fibo_imperative(n: u32) -> u32 {
    let mut res = 1;
    let mut prev = 0;

    if n == 0 { return 0; }

    for _ in 1..n {
        res += prev;
        prev = res - prev;
    }

    res
}

fn fibo_functional(n: u32) -> u32 {
    fn fibo_f_body(n0: u32, n1: u32, n: u32) -> u32 {
        if n == 0 { return n0; }
        fibo_f_body(n1, n0+n1, n-1)
    }
    fibo_f_body(0, 1, n)
}

fn fibo_test() {
    for i in 0..10 {
        println!("i : {}, f : {}", fibo_imperative(i), fibo_functional(i));
    }
}

fn facto_i(n: u32) -> u32 {
    let mut res = 1;

    for n in 1..n+1 {
        res *= n;
    }

    res
}

fn facto_f(n: u32) -> u32 {
    if n == 0 { return 1; }
    n * facto_f(n-1)
}

fn facto_test() {
    for i in 0..10 {
        println!("i : {}, f : {}", facto_i(i), facto_f(i));
    }
}
