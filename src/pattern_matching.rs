// ------------------------------
// Pattern matching
pub fn run() {
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
