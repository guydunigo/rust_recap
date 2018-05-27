// ------------------------------
// Fearless concurrency
pub fn run() {
    // ------------------------------
    // Threads
    {
        use std::thread;
        use std::time::Duration;

        // Creating a JoinHandle to keep track of the thread.
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("Hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("Hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();

        let v = vec![1, 2, 3];

        // You have to move the data you import in the Closure :
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }
    // ------------------------------
    // Channels (pipes)
    {
        use std::thread;
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        // Block the thread :
        println!("Got : {}", rx.recv().unwrap());
        // Doesn't :
        // println!("{}", rx.try_recv().unwrap());

        handle.join().unwrap();
    }
    // ------------------------------
    // Template
    {
    }
}
