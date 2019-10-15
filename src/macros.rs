// ------------------------------
// Macros, a quick intro
pub fn run() {
    // ------------------------------
    // Bases
    {
        // To import macros from crate serde
        // #[macro_use]
        // extern crate serde;
    }
    // ------------------------------
    // Macros definition like match :
    {
        #[macro_export]
        macro_rules! vec2 {
            ( $( $x:expr ),* ) => {
                {
                    let mut temp_vec = Vec::new();
                    $(
                        temp_vec.push($x);
                    )*
                    temp_vec
                }
            };
        }

        let a = vec2![3, 4];
        a.iter().map(|x| x * 2).for_each(|x| println!("{}", x));
    }
    // ------------------------------
    // Procedural macros for custom `derive`
    {
        // See : https://doc.rust-lang.org/stable/book/second-edition/appendix-04-macros.html
    }
}
