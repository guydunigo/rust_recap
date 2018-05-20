// ------------------------------
// Errors
pub fn run() {
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
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
        },
        Err(error) => panic!("Couldn't open the file : {:?}", error),
    };

    // unwrap on a Result will return the val in case of Ok(val)
    //  or panic! in case of Err
    let f = File::open("hello.txt").unwrap();

    // expect : Same with the error message
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    {
        use std::fs::File;
        use std::io;
        use std::io::Read;

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
