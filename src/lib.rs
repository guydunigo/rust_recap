//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculation more conveniont.

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(private_no_mangle_fns)]

mod basics;
mod ownership;
mod structs;
mod enums;
mod collections;
mod errors;
mod generics_traits_lifetimes;
pub mod tests;
mod functional;
mod smart_pointers;
mod concurrency;
mod oop;
mod pattern_matching;
mod unsafe_rust;
mod advanced_types;

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
        // functional::run();
    }
    // ------------------------------
    // Cargo
    {
        // See Cargo.toml and this file
        // Example at the bottom of this file and at the top
        // Compile the docs and open the browser : `$ cargo doc --open`
        // Examples are tested when executing : `cargo test`
        //
        // `pub use::submodule::subsubmodule::elmt` ...
        //  |=> create a simple interface for your crate's users

        // ------------------------------
        // Workspaces
        {
            // See workspaces folder
        }
    }
    // ------------------------------
    // Smart pointers : heap (Box<T>), Deref, Drop, Reference counters, ...
    {
        // smart_pointers::run();
    }
    // ------------------------------
    // Fearless concurrency : threads, channels (pipes), mutexes, some traits
    {
        // concurrency::run();
    }
    // ------------------------------
    // OOP and rust
    {
        // oop::run();
    }
    // ------------------------------
    // Pattern matching
    {
        // pattern_matching::run();
    }
    // ------------------------------
    // Unsafe : four places where `unsafe` is needed
    {
        // unsafe_rust::run();
    }
    // ------------------------------
    // Advanced types
    {
        advanced_types::run();
    }
    // ------------------------------
    // Template
    {
        template::run();
    }
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, rust_recap::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
