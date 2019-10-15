// ------------------------------
// Collections
pub fn run() {
    // ------------------------------
    // Vectors
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
            println!("{}", i);
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
            match *i {
                Types::Int(_) => println!("int"),
                Types::Text(_) => println!("text"),
                Types::Float(_) => println!("float"),
            }
        }
    }
    // ------------------------------
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
        for c in "नमस्ते".chars() {
            // visual caracters
            // println!("{}", c);
        }
        for c in "नमस्ते".bytes() {
            // integers
            // println!("{}", c);
        }
    }
    // ------------------------------
    // HashMaps
    {
        use std::collections::hash_map::Entry;
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        {
            let teams = vec![String::from("Blue"), String::from("Yellow")];
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
            {
                let count = match map.entry(word) {
                    Entry::Occupied(c) => c.into_mut(),
                    Entry::Vacant(case) => {
                        // Some heavy computation happening only once
                        case.insert(0)
                    }
                };
            }
            // or simpler :
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
