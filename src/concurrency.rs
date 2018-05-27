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
        use std::time::Duration;
        // Multiple producer, single consumer
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();
        let tx_2 = mpsc::Sender::clone(&tx);

        let handle = thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();

            for i in 0..5 {
                thread::sleep(Duration::from_millis(15));
                tx.send(i.to_string()).unwrap();
            }
        });

        let handle_2 = thread::spawn(move || {
            for i in 10..15 {
                thread::sleep(Duration::from_millis(15));
                tx_2.send(i.to_string()).unwrap();
            }
        });

        // Block the thread :
        println!("Got : {}", rx.recv().unwrap());

        loop {
            thread::sleep(Duration::from_millis(10));
            // Doesn't block :
            match rx.try_recv() {
                Ok(value) => println!("Got : {}", value),
                Err(mpsc::TryRecvError::Empty) => println!("Nothing yet."),
                Err(mpsc::TryRecvError::Disconnected) => {
                    println!("Channel disconnected, exiting...");
                    break;
                }
            }
        }

        handle.join().unwrap();
        handle_2.join().unwrap();
    }
    // ------------------------------
    // Share memory : Mutexes
    {
        use std::sync::{Mutex, Arc};
        use std::thread;

        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("m = {:?}", m);

        // use of Atomic ref counter :
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for i in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
    // ------------------------------
    // Template
    {
    }
}
