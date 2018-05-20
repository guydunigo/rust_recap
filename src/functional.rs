// ------------------------------
// Functional programming
pub fn run() {
    // ------------------------------
    // Closures
    {
            use std::thread;
            use std::time::Duration;
            // No types need to be specified but they can only be infered once
            // (if we call this closure with a u64,
            //  we won't be able to call it with String as well)
            let expensive_closure = |num| {
                println!("calculating slowly...");
                // thread::sleep(Duration::from_secs(2));
                num
            };

            // println!("{}", expensive_closure(4));

            use std::collections::HashMap;
            use std::collections::hash_map::Entry;
            use std::hash::Hash;

            struct Cacher<T, U>
                where T: Fn(U) -> U,
                      U: Copy + Eq + Hash
                      {
                          calculation: T,
                          values: HashMap<U, U> // Option<U>,
                      }

            impl<T, U> Cacher<T, U>
                where T: Fn(U) -> U,
                      U: Copy + Eq + Hash
                      {
                          fn new(closure: T) -> Cacher<T, U> {
                              Cacher {
                                  calculation: closure,
                                  values: HashMap::new()
                              }
                          }

                          fn value(&mut self, arg: U) -> U {
                              match self.values.entry(arg) {
                                  Entry::Occupied(v) => v.get().clone(),
                                  Entry::Vacant(value) => {
                                      let v = (self.calculation)(arg);
                                      value.insert(v);
                                      v
                                  }
                              }
                              // match self.values.get(&arg) {
                              //     Some(v) => v.clone(),
                              //     None => {
                              //         let v = (self.calculation)(arg);
                              //         self.values.insert(arg, v);
                              //         v
                              //     },
                              // }
                          }
                      }

            let mut ec = Cacher::new(expensive_closure);
            println!("{}", ec.value(3));
            println!("{}", ec.value(3));
            println!("{}", ec.value(4));
            println!("{}", ec.value(3));

            {
                let capturer = |num| ec.value(num);
            }
            // traits :
            // FnOnce : take ownership of the outside variables it uses
            // FnMut : mutable ref of the ouside vars it uses
            // Fn : immutable ref of ...
            {
                let capturer_that_moves_ec = move |num| ec.value(num);
            }
        }
    // ------------------------------
    // Iterators
    {
        // Iterators are lazy !
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        for val in v1_iter {
            println!("Got {}", val);
        }

        v1.iter().for_each(|val| println!("Got {}", val));
        let sum: i32 = v1.iter().sum();
        println!("Sum : {}", sum);

        // iterators are lazy so are useless unless consumed :
        // without the collect, nothing is computed.
        let vec_add: Vec<i32> = v1.iter().map(|x| x + 1).collect();

        // Implementing Iterator trait
        struct Counter {
            count: u32,
        }

        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }

        impl Iterator for Counter {
            type Item = u32;
            fn next(&mut self) -> Option<u32> {
                if self.count < 5 {
                    self.count += 1;
                    Some(self.count)
                } else {
                    None
                }
            }
        }

        for c in Counter::new() {
            println!("Counter : {}", c);
        }

        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        println!("Sum : {}", sum);
    }
    // ------------------------------
}
