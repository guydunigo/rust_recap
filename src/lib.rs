#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod basics;
mod ownership;
mod structs;
mod enums;
mod collections;
mod errors;
mod generics_traits_lifetimes;
pub mod tests;
mod functional;

mod template;
mod divers;

pub fn run() {
    // ------------------------------
    // Basics
    {
        // Variables
        // Types
        // Tuples
        // Functions, expressions (without ';', return a value), statements
        // Control flow

        // basics::run();
    }
    // ------------------------------
    // Ownership, moving, borrowing, slices
    {
        // ownership::run();
    }
    // ------------------------------
    // Structs
    {
        // structs::run();
    }
    // ------------------------------
    // Enums
    {
        // enums::run();
    }
    // ------------------------------
    // Collections : Vectors, Strings (stored in UTF-8), HashMaps
    {
        // collections::run();
    }
    // ------------------------------
    // Errors
    {
        // errors::run();
    }
    // ------------------------------
    // Generics, Traits, Lifetimes
    {
        // generics_traits_lifetimes::run();
    }
    // ------------------------------
    // Tests
    {
        // See `test.rs` for unit tests and `tests` folder for integration tests.
        // `$ cargo test`
    }
    // ------------------------------
    // Functional programming : Closures, Iterators
    {
        functional::run();
    }
    // ------------------------------
    // Template
    {
        template::run();
    }
} 