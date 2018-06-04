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
    // Template
    {
    }
}
