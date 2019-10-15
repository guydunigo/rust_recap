// ------------------------------
// Advanced types
pub fn run() {
    // ------------------------------
    // Type aliases
    {
        type Kilometers = i32;
        type Thunk = Box<dyn Fn() + Send + 'static>; // You wouldn't repeat that everywhere, would you ?
        let f: Thunk = Box::new(|| println!("hi"));
        fn takes_long_type(f: Thunk) {
            f();
        }
        takes_long_type(f);
    }
    // ------------------------------
    // !, the type that never returns
    {
        // fn bar() -> ! {
        // }

        // It allows us to use like here a `continue` or `panic!` in a match (which requires
        // returning the same type in all branches

        // Here, continue has a value of !
        // let guess: u32 = match guess.trim().parse() {
        //         Ok(num) => num,
        //         Err(_) => continue,
        // };

        // Panic! "is" a ! as well
        // loop {} as well
    }
    // ------------------------------
    // Dynamically sized types and `Sized`
    {
        // DST like str have to be stored in &str, Box<str>, Rc<str>
        // Same for trait objects

        // The trait `Sized` is given to every type that have a known size at compile time.
        // Is is passed by default to generic parameters :
        // fn generic<T>(t: T) {} == fn generic<T: Sized>(t: T) {}
        //
        // To disable that :
        // fn generic<T: ?Sized>(t: &T) {}
    }
}
