#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    // ------------------------------
    // Variables
    {
        // Variable mutable (otherwise constants) only with `mut`
        let mut x = 5;
        // Actual constants, must have a type defined
        const Y: u32 = 100_000;
        println!("x = {}", x);
        x = 6;
        println!("x = {}", x);
        // Shadow variables
        let x = "a";
    }
    // ------------------------------
    // Types
    {
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
    }
    // ------------------------------
    // Tuples
    {
        let tup: (i32, f64, u8) = (500, 6.4, 1);

        // Destructuring
        let (x, y, z) = tup;
        let x = tup.0;
        let y = tup.1;
        let z = tup.2;

        let arr = [1, 2, 3, 4, 5];
        let x = arr[3];
    }
    // ------------------------------
    // Functions, expressions (without ';', return a value), statements
    {
        fn function_sample(x: i32) {
            println!("Another function. {}", x);
        }
        fn five() -> i32 {
            5 // no ';' because only expressions return a value
        }

        let x = 5;
        function_sample(x);

        let y = {
            let x = 3;
            x + 1 // expression that is returned from the block
        };
    }
    // ------------------------------
    // Control flow
    {
        let x = 5;
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
    }
    // ------------------------------
    // Ownership, moving, borrowing, slices
    {
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
    }
    // ------------------------------
    // Structs
    {
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
    }
    // ------------------------------
    // Enums
    {
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
        else {
            println!("not three");
        }
        // Same as :
        match i3 {
            Some(3) => println!("three"),
            _ => println!("not  three"),
        }
    }
    // ------------------------------
    // Collections
    {
        // Enums
        {
            let v: Vec<i32> = Vec::new();
            let mut v = vec![1, 2, 3];
            v.push(3);
            v.pop();

            let v = vec![1, 2, 3, 4, 5];

            let third: i32 = v[2];
            let third_r: &i32 = &v[2];
            let third_o: Option<&i32> = v.get(2);
            println!("{} {}", third, v[2]);

            let mut v = vec![1, 2, 3, 4, 5];

            for i in &mut v {
                *i *= 50;
            }
            for i in &v {
                println!("{}",i);
            }

            // To store multiple types in a single vector, use enums :
            enum Types {
                Int(i32),
                Float(f64),
                Text(String),
            }

            let row = vec![
                Types::Int(3),
                Types::Text(String::from("blue")),
                Types::Float(3.2),
            ];
            for i in &row {
                match i {
                    &Types::Int(_) => println!("int"),
                    &Types::Text(_) => println!("text"),
                    &Types::Float(_) => println!("float"),
                }
            }
        }
        // Strings, stored in UTF-8
        {
            let hello = String::from("السلام عليكم");
            let hello = String::from("Dobrý den");
            let hello = String::from("Hello");
            let hello = String::from("שָׁלוֹם");
            let hello = String::from("नमस्ते");
            let hello = String::from("こんにちは");
            let hello = String::from("안녕하세요");
            let hello = String::from("你好");
            let hello = String::from("Olá");
            let hello = String::from("Здравствуйте");
            let hello = String::from("Hola");

            let s = String::new();
            let s = "test".to_string();
            let mut s = String::from("test");

            s.push_str("bar");
            s.push('b');

            // (+) only between a String and &str so the first string will be moved !
            let s1 = String::from("Hello, ");
            let s2 = String::from("world!");

            let s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used

            // format! macro doesn't move the parameters
            let s1 = String::from("tic");
            let s2 = String::from("tac");
            let s3 = String::from("toe");

            let s = format!("{}-{}-{}", s1, s2, s3);

            // Because of UTF-8, you can't index the string (i.e. s[2])
            for c in "नमस्ते".chars() { // visual caracters
                // println!("{}", c);
            }
            for c in "नमस्ते".bytes() { // integers
                // println!("{}", c);
            }
        }
        // HashMaps
        {
            use std::collections::HashMap;

            let mut scores = HashMap::new();
            scores.insert(String::from("Blue"), 10);
            scores.insert(String::from("Yellow"), 50);

            {
                let teams  = vec![String::from("Blue"), String::from("Yellow")];
                let initial_scores = vec![10, 50];

                // HashMap<_, _> is replaced by HashMap<&String, &i32> :
                let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
            }

            {
                let score: Option<&i32> = scores.get(&String::from("Blue"));
                if let Some(i) = score {
                    println!("{}", i);
                }
            }

            // ------------------------------
            // modify a value :

            // Overwrite
            scores.insert(String::from("Blue"), 32);

            // insert if no value for the key :
            scores.entry(String::from("Yellow")).or_insert(32);

            // Modify based on the previous value :
            let text = "hello world wonderful world";
            let mut map = HashMap::new();
            for word in text.split_whitespace() {
                // Very useful to modify the value !
                //  (with getting shouted at by the borrow checker)
                let count = map.entry(word).or_insert(0);
                *count += 1;
            }

            // ------------------------------
            // loop through keys and values
            for (key, value) in &scores {
                println!("{}: {}", key, value);
            }
        }
    }
    // ------------------------------
    // Errors
    {
        // panic! = unrecoverable error, program crashes
        // panic!("crash and burn");

        // Recoverable errors :
        // enum Result<T, E> {
        //     Ok(T),
        //     Err(E),
        // }

        use std::fs::File;
        use std::io::ErrorKind;

        let f_result = File::open("hello.txt");
        match f_result {
            Ok(file) => file,
            Err(ref error) if error.kind() == ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => {
                        panic!(
                            "Tried to create file but there was a problem: {:?}",
                            e
                            )
                    },
                }
            },
            Err(error) => panic!("Couldn't open the file : {:?}", error),
        };

        // unwrap on a Result will return the val in case of Ok(val)
        //  or panic! in case of Err
        let f = File::open("hello.txt").unwrap();

        // expect : Same with the error message
        let f = File::open("hello.txt").expect("Failed to open hello.txt");

        {
            use std::io;
            use std::io::Read;
            use std::fs::File;

            fn read_username_from_file() -> Result<String, io::Error> {
                let f = File::open("hello.txt");

                let mut f = match f {
                    Ok(file) => file,
                    Err(e) => return Err(e),
                };

                let mut s = String::new();

                match f.read_to_string(&mut s) {
                    Ok(_) => Ok(s),
                    Err(e) => Err(e),
                }
            }

            // Appending the question mark '?' to the function call automatically propagates
            // (returns) the error message in case of Result::Err
            // Only for functions returning a Result
            fn read_username_from_file_2() -> Result<String, io::Error> {
                let mut f = File::open("hello.txt")?;
                let mut s = String::new();
                f.read_to_string(&mut s)?;
                Ok(s)
            }
            fn read_username_from_file_3() -> Result<String, io::Error> {
                let mut s = String::new();
                File::open("hello.txt")?.read_to_string(&mut s)?;
                Ok(s)
            }
        }
    }
    // ------------------------------
    // Generics, Traits, lifetimes
    {
        {
            // ------------------------------
            // Generics
            fn largest<T: PartialOrd>(list: &[T]) -> &T {
                let mut largest = &list[0];

                for item in list.iter() {
                    if item > largest {
                        largest = item;
                    }
                }

                largest
            }

            let list = [0, 2, 4, 1, 5, 6, 7, 642, 914, 66, 75];
            println!("The largest of the list : {}.", largest(&list));

            struct Point<T> {
                x: T,
                y: T,
            }

            impl<T> Point<T> {
                fn x(&self) -> &T {
                    &self.x
                }
            }

            impl Point<f32> {
                fn distance_from_origin(&self) -> f32 {
                    (self.x.powi(2) + self.y.powi(2)).sqrt()
                }
            }
        }
        // ------------------------------
        // Traits
        {
            trait Summarizable {
                fn summary(&self) -> String;
                fn default(&self) -> String {
                    format!("Default behavior for this method : {}", self.summary())
                }
            }

            struct Novel {
                text: String
            }

            impl Summarizable for Novel {
                fn summary(&self) -> String {
                    format!("{}...", &self.text[0..40])
                }
            }

            fn get_summary<T: Summarizable>(text: &T) {
                println!("{}", text.summary());
            }

            let nov = Novel {
                text: String::from(
                          "This is a great story with very interesting characters. They go through some long and weary adventure wich make them hate each other at first, but they eventually get back together and end even closer than they began. The end !"
                          )
            };
            get_summary(&nov);
            println!("{}", nov.default());

            // With generics :
            trait Testable<T> {
                fn test(&self, t: T) -> (T,&T);
            }
            struct Tested<T> {
                x: T
            }

            impl<T> Testable<T> for Tested<T> {
                fn test(&self, t: T) -> (T,&T) {
                    (t, &self.x)
                }
            }

            let test_var = (Tested { x: 5 }).test(4);
            println!("{} {}", test_var.0, test_var.1);

            // Using multiple traits at the same time
            fn some_function<T: Testable<u32> + Clone, U: Clone + Summarizable>(t: T, u: U) -> i32 { 3 }
            fn some_function_2<T, U>(t: T, u: U) -> i32
                where T: Testable<u32> + Clone,
                      U: Clone + Summarizable
                      { 3 }

            // Defining methods only for certain traits
            use std::fmt::Display;

            struct Pair<T> {
                x: T,
                y: T,
            }

            impl<T> Pair<T> {
                fn new(x: T, y: T) -> Self {
                    Self {
                        x,
                        y,
                    }
                }
            }

            impl<T: Display + PartialOrd> Pair<T> {
                fn cmp_display(&self) {
                    if self.x >= self.y {
                        println!("The largest member is x = {}", self.x);
                    } else {
                        println!("The largest member is y = {}", self.y);
                    }
                }
            }

            // Blanket implementations
            // Implements the ToString trait for all types implementing Display :
            // impl<T: Display> ToString for T {}
        }
        // ------------------------------
        // Lifetimes :
        {
            // name starting with an apostroph : 'a
            // The return value must, at maximum, have the lifetime of the shortest lifetime
            // of the acguments
            fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
                if string1.len() > string2.len() {
                    string1
                }
                else {
                    string2
                }
            }

            let string1 = String::from("abcd");
            let string2 = "xyz";

            let result = longest(string1.as_str(), string2);
            println!("The longest string is {}", result);

            // Refs in structs need lifetines :
            struct ImportantExcept<'a> {
                part: &'a str,
            }

            fn test<'a, 'b>(a: &'a str, b: &'b str) -> (&'a str, &'b str) {
                (a, b)
            }

            // 'static lifetime : whole program
        }
        // Summary
        use std::fmt::Display;
        fn longest_with_as_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
            where T: Display
            {
                println!("Announcement! {}", ann);
                if x.len() > y.len() {
                    x
                } else {
                    y
                }
            }
    }
    // ------------------------------
    // Tests
    {
        // See `test` crate.
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
