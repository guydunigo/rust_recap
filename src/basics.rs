// ------------------------------
// Basics
pub fn run() {
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
        } else {
            println!(">= 5");
        }

        loop {
            break;
        }

        let mut i = 0;
        while i != 3 {
            i = i + 1;
        }

        let a = [10, 20, 30, 40, 50];

        for element in a.iter() {
            println!("the value is: {}", element);
        }

        // Ranges :
        //   1..4 == 1, 2, 3
        //   1..=4 == 1, 2, 3, 4
        for element in (1..4).rev() {
            println!("the value is: {}", element);
        }
    }
}
