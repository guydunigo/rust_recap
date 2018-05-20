// ------------------------------
// Ownership, moving, borrowing, slices
pub fn run() {
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
