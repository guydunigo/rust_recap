// ------------------------------
// Generics, Traits, Lifetimes
pub fn run() {
    // ------------------------------
    // Generics
    {
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
            text: String,
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
            fn test(&self, t: T) -> (T, &T);
        }
        struct Tested<T> {
            x: T,
        }

        impl<T> Testable<T> for Tested<T> {
            fn test(&self, t: T) -> (T, &T) {
                (t, &self.x)
            }
        }

        let test_var = (Tested { x: 5 }).test(4);
        println!("{} {}", test_var.0, test_var.1);

        // Using multiple traits at the same time
        fn some_function<T: Testable<u32> + Clone, U: Clone + Summarizable>(t: T, u: U) -> i32 {
            3
        }
        fn some_function_2<T, U>(t: T, u: U) -> i32
        where
            T: Testable<u32> + Clone,
            U: Clone + Summarizable,
        {
            3
        }

        // Defining methods only for certain traits
        use std::fmt::Display;

        struct Pair<T> {
            x: T,
            y: T,
        }

        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
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
            } else {
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

        // ------------------------------
        // Lifetime subtyping ensures one lifetime outlives another
        {
            struct Context<'s>(&'s str);

            // 's is at least as long as 'c
            struct Parser<'c, 's: 'c> {
                context: &'c Context<'s>,
            }

            impl<'c, 's> Parser<'c, 's> {
                fn parse(&self) -> Result<(), &'s str> {
                    Err(&self.context.0[1..])
                }
            }

            fn parse_context<'s>(context: Context<'s>) -> Result<(), &'s str> {
                Parser { context: &context }.parse()
            }
        }
        // ------------------------------
        // Lifetime bounds on references to feneric type
        {
           struct Ref<'a, T>(&'a T) where T: 'a;
        }
        // ------------------------------
        // Inference of trait object lifetimes
        {
            trait Red {}

            struct Ball<'a> {
                diameter: &'a i32,
            }

            impl<'a> Red for Ball<'a> {}

            let num = 5;
            let obj = Box::new(Ball { diameter: &num }) as Box<Red>;
        }
    }
    // ------------------------------
    // Summary
    use std::fmt::Display;
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
