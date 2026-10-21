//! ============================================================================
//! The Rust Programming Language — Chapter 16: Fearless Concurrency
//! https://doc.rust-lang.org/book/ch16-00-concurrency.html
//!
//!   16.1  Using Threads to Run Code Simultaneously
//!   16.2  Transfer Data Between Threads with Message Passing
//!   16.3  Shared-State Concurrency
//!   16.4  Extensible Concurrency with `Send` and `Sync`
//! ============================================================================

fn main() {
    // THREADS

    /* Imagine threads as multiple employees working simultaneously on
    different tasks.
    A programs code runs inside a process, within that process threads
    allow independent tasks to execute at the same exact time.

    A web server uses threads to handle multiple requests,
    it doesn't wait to serve one and serve the other after it's done.

    By splitting heavy duties across multiple cores, we can finish tasks
    much faster than ogin them one by one in a single line.


    CHAOS:
    Multiple threads can cause chaos like two processes
    reading/writing to a shared datasheet, task A might change something
    after Task B has read and started working on it causing outdated information

    Deadlocks: thread A is waiting for a resource locked by Thread B, while
    Thread B is waiting for a resource locked by Thread A, Both get stuck forever

    Operating systems provide APIs to spin up threads.
    THE 1:1 MODEL - Every single thread we spawn in Rust corresponds to one real operating system thread.
        This relies on OS's battle tested scheduling, low runtime overhead and simple.
        But
        Os threads are heavy on memory and resources so we cant use millions at once.

    M:N MODEL-Several third party libraries that offer green threads, where
    many language level tasks M map to a smaller pool of operating system threads N.

    Async/Await - RUST also offers an async system, which lets a single thread pause
    and deal with many waiting tasks like waiting for network responses without creating a full OS
    thread for each one.





    Creating a new thread with spawn.

    */


    use std::thread;
    use std::time::Duration;

    thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned {i} threads");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!(" spawned {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }

    // this spawns threads from 1 to 10 inside our closure function
    // and spawns main threads from 1 to 5 in our main
    // THe output of this can tell us how less overhead we have with OS threads
    /* spawned 1 from main thread
Spawned 1 threads
 spawned 2 from main thread
Spawned 2 threads
Spawned 3 threads
 spawned 3 from main thread
 spawned 4 from main thread
Spawned 4 threads
Spawned 5 threads

WE can see in this output the OS scheduler decided to give cpu time to a main thread
even though in our code it's clearly the after our regular threads.

We can also see that our regular threads stopped at 5 even though it is 1..10
that is because our main(), the main thread finished at 5, therefore ending the fn main()
in Rust when fn main() ends the entire program halts.

   */

        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle.join().unwrap();// sequential

        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap(); // concurrent
    /*
    JOIN().
    A join handle is an owned value that represents our background thread.
    If we run .join() we tell the current thread to stop and wait until
    other thread is completely finished.
    This is also call BLOCKING, when a thread is blocked, its CPU execution time is done

    we use unwrap, because .join return a Result.
    Ok(value) the thread completed successfully and optionally returned a value.
    Err(err) the thread panicked while running.
    this is for the background thread, unwrap handles the result.


    CONCURRENT VS SEQUENTIAL CALL OF .JOIN

    Concurrent - .join at the end()
        the code above, when .join is at the end the main thread spawn
        finishes and when normally it would terminate the whole main()
        and end the background thread spawn, it waits for the background thread
        then program finished

    Sequential - .join at the beginning() of main thread pawn.
        .join at the beginning is stopping the main thread before it starts
        waits for the background thread spawn to finish then continues its own
        until its goal.
        no actual concurrency occurs, the threads are run in sequence.

     */

    /*
    using MOVE closures with Threads
    Rust rejects borrowed data across threads, and we can use move to fix it.

    Closures default to borrowing, if they only need to read them
     */

    let v = vec![1,2,3];
    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}") // closure tries to borrow &v
    });
    /*
    because println! only reads v, closure borrows v but the compiler throws
    an error, "closures may outlive the current function, but it borrows v"
    meaning v might not live while closure lives more than v causing a null
    memory pointer.
    Compiler also doest know how long a thread will run, it could run after main() finishes
    which is also garbage memory.

     */
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}"); // Borrowing v
    });

    drop(v); // Main thread instantly destroys v from memory!

    handle.join().unwrap();

    /* this code is an example where the OS pauses the thread before it runs
    and drops v, coniferous the thread trying to read garbage memory
    now we can fix this with MOVE

    Fix: tell rust tog ive the entire variable to the new thread instead of
    borrowing it
     */

    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        println!("here's a vector {v:?}")
    });
    handle.join().unwrap();

    /* pushing move info of the || takes full ownership of v     is physically out of the main threads scope and into the spawned threads
    environment
    v will stay alive as long the spawned thread exists, and will be cleaned up
    automatically after the thread finished.

    main() has no access to v, cant drop, or change.
    if both threads need the data we need to .clone() so each thread
    has its own independent copy.
    or use thread safe shared ownership types like
    Arc<T> and Mutex<T>

     */



    /* Transferring data between THREADS
    instead of having threads try to fight over the same memory, message passing
    allows threads to communicate eby sending pieces of data to each other.
    Go language principle - do not communicate by sharing memory; instead,
    share memory by communicating.

    A channel is like a one-way river:
    Transmitter(tx) - upstream, where we drop data into the river.
    Receiver(rx) - downstream, where we wait to catch the data.
    If either rx or tx is dropped, channel is closed.

    MPSC - std::sync::mpsc what does it mean?
    Mpsc stands for Multiple Producer, Single Consumer
    Multiple producers - we can have many threads holding transmitters,
    sending data into the same channel
    Single Consumer - there can only ever be ONE receiving and consuming
    these messages.
     */
    use std::sync::mpsc;
    let (tx,rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("HI");
        tx.send(val).unwrap();
    });

    /* mpsc::channel constructs the channel and returns a tuple (transmitter, receiver)
       move moves tx out of the main thread into spawned thread which now owns tx.
       tx.send(val) puts the message into the channel.
       send() takes full ownership of val
       send() also return Result<(), SendError<T>> if the receiving end
       rx has already been dropped/destroyed sending fails, because nobody
       to receive it.
       we call .unwrap() if an error occurs to handle it.
       tx automatically gets the .send() because it its attained a transmission
     */

    let received =rx.recv().unwrap();
    println!("received: {received}");
    /* Above is printing the received message, there is two ways to grab data
    rx.recv() - Pauses the current thread completely until a message arrives,
    we should use this when the thread has nothing else to do except wait for data.
    rx.try_recv() - checks instantly, returns OK(message) if one is waiting err if
     When the thread has other work to do like running a loop or UI, and jus wants to check
     for data periodically.
     this one doesn't block. it just checks.
     */

    let val1 = String::from("Hi");
    tx.send(val).unwrap();
    println!("val is {val}");

    /* ths would give a compiler value because in rust sending an object
    down a queue/data stream passes its ownership unlike other languages.
    In other languages THREAD A could modify while THREAD B holds pointer and rememberers it differently
    rusts solves this by taking ownership of the argument.
     */

    thread::spawn(move || {
        let vals = vec![String::from("hi"),
                        String::from("hi2"),
                        String::from("hi3"),
                        String::from("hi4")];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_sec(2));
        }
    });

    for received in rx {
        println!("{received}")
    }

    /*
    above is sending multiple values with a simple loop through vec/     is empty at the end.

    and instead of calling rx.recv().unwrap() everytime we can treat the rx
    as an iterator.
    on each iteration of the loop it blocks the thread wanna It's for msg.
    if no msg is present loop blocks until one arrives .
     */






}
