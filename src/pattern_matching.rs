// ------------------------------
// Pattern matching
pub fn run() {
    // ------------------------------
    // Ways to match
    {
        let var = Some(3);
        match var {
            Some(3) => println!("this is 3"),
            None => println!("none"),
            _ => println!("else"),
        }

        if let Some(3) = var {
            println!("this is 3");
        } else if let None = var {
            // ...
        }

        let mut stack = vec![1,422,5];
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }

        let stack = vec![1,422,5];
        for (index, value) in stack.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }

        let (x, y, z) = (1, 2, 3);

        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({}, {})", x, y);
        }
        let point = (3, 5);
        print_coordinates(&point);

    }
    // ------------------------------
    // Matching helpers
    {
        let x = 10;
        match x {
            1 => println!("One"),
            // match multiple patterns :
            4 | 6 => println!("Four or six"),
            // ranges : (4 and 10 included)
            4 ... 10 => println!("Small value"),
            _ => println!("Any"),
        }

        let c = 'c';
        match c {
            'a' ... 'j' => println!("early letter"),
            'k' ... 'z' => println!("late letter"),
            _ => println!("something else"),
        }

        let y = Some(46);

        match y {
            Some(44) => println!("44"),
            // Create a new x variable :
            Some(x) => println!("{}", x),
            // Some(n) => println!("{}", n),
            None => println!("None"),
        }
    }
    // ------------------------------
    // Destructuring
    {
        // tuples
        let (x, y, z) = (1, 2, 3);

        // Structs
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b } = p;
        let Point { x, y } = p;
        match p {
            Point { x: 0, y } => println!("Axe des abscisses"),
            Point { x, y: 0 } => println!("Axe des ordonnées"),
            Point { x: _, y } => println!("Something "),
        }

        // Enums
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        let msg = Message::ChangeColor(0, 160, 255);
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            },
            Message::Move { x, y } => {
                println!(
                    "Move in the x direction {} and in the y direction {}",
                    x,
                    y
                    );
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!(
                    "Change the color to red {}, green {}, and blue {}",
                    r,
                    g,
                    b
                    )
            }
        }

        // references
        let points = vec![ Point { x: 0, y: 0 } ];
        let sum_of_squares: i32 = points
            .iter()
            .map(|&Point { x, y }| x * x + y * y)
            .sum();

        // Combinated
        let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
    }
    // ------------------------------
    // Ignoring
    {
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {}", y);
        }

        // Disable Rust unused var warning :
        let _x = 4;

        // Ignore remaining values
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }
        let origin = Point { x: 0, y: 753, z: 3 };
        match origin {
            Point { x, .. } => println!("x is {}", x),
        }

        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            },
        }
    }
    // ------------------------------
    // ref and ref mut
    {
        let mut robot_name = Some(String::from("Bors"));
        match robot_name {
            Some(ref name) => println!("Found a name {}", name),
            None => (),
        }
        println!("robot_name is: {:?}", robot_name);

        match robot_name {
            Some(ref mut name) => *name = String::from("Another name"),
            None => (),
        }
        println!("robot_name is: {:?}", robot_name);
    }
    // ------------------------------
    // Match guards
    {
        let nul = Some(5);
        match nul {
            Some(x) if x < 7 => println!("Less than five {}", x),
            Some(x) => println!("Whatever..."),
            None => (),
        }

        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(1) | Some(4) if y >= 35 => println!("aoue"),
            Some(n) if n == y => println!("Matched, n = {:?}", n),
            _ => println!("Default case, x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {:?}", x, y);
    }
    // ------------------------------
    // @ : to have condition and place into a variable
    {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello { id: id_variable @ 3...7 } => {
                println!("Found an id in range: {}", id_variable)
            },
            Message::Hello { id: 10...12 } => {
                println!("Found an id in another range")
            },
            Message::Hello { id } => {
                println!("Found some other id: {}", id)
            },
        }
    }
}
