// ------------------------------
// Smart pointers
pub fn run() {
    // ------------------------------
    // Allocate on the heap : Box<T>
    {
        let b = Box::new(5);
        println!("b = {}", b);

        // Doesn't compile :
        // struct IterList {
        //     l: Box<List>,
        // }
        // impl IterList {
        //     pub fn new(l: List) -> IterList {
        //         IterList { l: Box::new(l) }
        //     }
        // }

        // impl Iterator for IterList {
        //     type Item = i32;

        //     fn next(&mut self) -> Option<i32> {
        //         match self.l {
        //             box List::Cons(v, sub) => {
        //                 self.l = sub;
        //                 Some(*v)
        //             },
        //             box List::Nil => None,
        //         }
        //     }
        // }

        struct IterList<'a> {
            l: &'a List,
        }
        impl<'a> IterList<'a> {
            pub fn new(l: &'a List) -> IterList {
                IterList { l }
            }
        }
        impl<'a> Iterator for IterList<'a> {
            type Item = i32;

            fn next(&mut self) -> Option<i32> {
                match self.l {
                    List::Cons(v, sub) => {
                        self.l = sub;
                        Some(*v)
                    }
                    List::Nil => None,
                }
            }
        }

        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        impl List {
            pub fn new(value: i32, sub_list: List) -> List {
                List::Cons(value, Box::new(sub_list))
            }
            pub fn from_vec(values: Vec<i32>) -> List {
                values
                    .iter()
                    .rev()
                    .fold(List::Nil, |res, v| List::new(*v, res))
            }
            pub fn value(&self) -> Option<i32> {
                match self {
                    List::Cons(v, _) => Some(*v),
                    List::Nil => None,
                }
            }
            pub fn sub(&self) -> Option<&List> {
                match self {
                    List::Cons(_, sub) => Some(&sub),
                    List::Nil => None,
                }
            }
            pub fn iter(&self) -> IterList {
                IterList::new(self)
            }
        }

        // use List::{Cons, Nil};
        let list = List::new(1, List::new(2, List::new(3, List::Nil)));
        let vec = vec![1, 2, 3];
        let list2 = List::from_vec(vec);
        if let Some(v) = list.value() {
            print!("{} ", v);
        }
        if let Some(sub) = list2.sub() {
            if let Some(v) = sub.value() {
                println!("{}", v);
            }
        }
        list2.iter().for_each(|x| println!("{}", x));
    }
    // ------------------------------
    // Deref
    {
        let x = 5;
        let y = &x;
        let z = Box::new(x);

        assert_eq!(x, 5);
        assert_eq!(*y, 5);
        assert_eq!(*z, 5);

        struct MyBox<T>(T);

        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        use std::ops::Deref;
        impl<T> Deref for MyBox<T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.0
            }
        }

        let a = MyBox::new(4);
        println!("{}", *a);
        fn hello(name: &str) {
            println!("Hello {}!", name);
        }
        let str = MyBox::new("Test");
        hello(&str);
    }
    // ------------------------------
    // Drop : The destructor !
    {
        struct CustomSmartPointer {
            data: String,
        }

        impl Drop for CustomSmartPointer {
            fn drop(&mut self) {
                println!("Dropping CustomSmartPointer with data `{}`!", self.data);
            }
        }

        {
            let c = CustomSmartPointer {
                data: String::from("my stuff"),
            };
            println!("CustomSmartPointer created.");
            // then goes out of scope so the drop function is automatically called
        }
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        println!("CustomSmartPointer created.");
        // Manually dropped :
        drop(c);
    }
    // ------------------------------
    // Reference counter (doesn't clear memory unless nothing use it anymore)
    {
        struct IterList<'a> {
            l: &'a List,
        }
        impl<'a> IterList<'a> {
            pub fn new(l: &'a List) -> IterList {
                IterList { l }
            }
        }
        impl<'a> Iterator for IterList<'a> {
            type Item = i32;

            fn next(&mut self) -> Option<i32> {
                match self.l {
                    List::Cons(v, sub) => {
                        self.l = sub;
                        Some(*v)
                    }
                    List::Nil => None,
                }
            }
        }

        use std::rc::Rc;

        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }

        impl List {
            pub fn new(value: i32, sub_list: Rc<List>) -> List {
                List::Cons(value, sub_list)
            }
            pub fn from_vec(values: Vec<i32>) -> List {
                values
                    .iter()
                    .rev()
                    .fold(List::Nil, |res, v| List::new(*v, Rc::new(res)))
            }
            pub fn value(&self) -> Option<i32> {
                match self {
                    List::Cons(v, _) => Some(*v),
                    List::Nil => None,
                }
            }
            pub fn sub(&self) -> Option<&List> {
                match self {
                    List::Cons(_, sub) => Some(&sub),
                    List::Nil => None,
                }
            }
            pub fn iter(&self) -> IterList {
                IterList::new(self)
            }
        }

        let a = Rc::new(List::from_vec(vec![5, 10, 42]));
        let b = List::new(3, Rc::clone(&a));
        let c = List::new(4, Rc::clone(&a));
        a.iter().for_each(|x| print!("{} ", x));
        println!();
        b.iter().for_each(|x| print!("{} ", x));
        println!();
        c.iter().for_each(|x| print!("{} ", x));
        println!();
    }
    // ------------------------------
    // RefCell : Make unsafe things that are checked at execution rather than at compile time
    {
        // See below
        // borrow from a RefCell with : .borrow() and .borrow_mut()
        // Applies the borrowing rules (many unmutable or ONE mutable ref) at runtime
        // and panic! if there is a problem.

        // Mixing RefCell with Rc to obtain multiple references
        // And be able to mutate the variable

        #[derive(Debug)]
        enum List {
            Cons(Rc<RefCell<i32>>, Rc<List>),
            Nil,
        }

        use std::cell::RefCell;
        use std::rc::Rc;

        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));

        let b = List::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = List::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
    // ------------------------------
    // Ref Cycle : the Rc strong_count never goes to 0 by itself so the memory is lost.
    {
        use std::cell::RefCell;
        use std::rc::Rc;

        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }

        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match *self {
                    List::Cons(_, ref item) => Some(item),
                    List::Nil => None,
                }
            }
        }

        let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));

        // Rc::strong_count(myrc) : gives the number of references to the data
        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // overflow warning, we have a loop :
        // println!("a next item = {:?}", a.tail());
    }
    // ------------------------------
    // Weak pointers :
    // obtained from Rc::downgrade(myrc), raise the weak_count instead of the strong_count
    // weak_count doesn't need to be zero for the data to be dropped
    // To check if it has been dropped and use it, call the method `upgrade` to get a Option<Rc<T>>
    {
        use std::cell::RefCell;
        use std::rc::{Rc, Weak};

        #[derive(Debug)]
        struct Node {
            value: i32,
            children: RefCell<Vec<Rc<Node>>>,
            parent: RefCell<Weak<Node>>,
        }

        let leaf = Rc::new(Node {
            value: 3,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        });

        let branch = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("Leaf's value : {}", leaf.value);
        println!("Branch's value : {}", branch.value);
        branch
            .children
            .borrow()
            .iter()
            .for_each(|l| println!("Branch's child's value : {}", l.value));

        let pp = leaf.parent.borrow().upgrade();
        if let Some(parent) = pp {
            println!("Leaf's parent's value : {}", parent.value);
        }
        // no memory leak due to data cycling thanks to the Weak<T>
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
