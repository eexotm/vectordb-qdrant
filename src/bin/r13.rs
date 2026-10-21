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

    /* Imagine threads as multiple employees working simutaenously on
    different tasks.
    A programs code runs inside a process, within that process threads
    allow independtend tasks to exectue at the same exact time.

    A web server uses threads to handle multiple requests,
    it doesnt wait to serve one and serve the other after its done.

    By splitting heavy duties across multiple cores, we can finish tasks
    musch faster thand ogin them oneby one in a single line.


    CHAOS:
    Multiple threads can cause chaos like two processes
    reading/writing to a shared datasheet, task A might change somethign
    after Task B has read and started working on it causing outdated informaiton

    Deadlocks: thread A is waiting for a resource locked by Thread B, while
    Thread B is waiting for a resource locked by Thread A, Both get stuck forevre

    Operating systems provide APIs to spin up threads.
    THE 1:1 MODEL - Every single thread we spawn in Rust corresponds to one real operation system thread.
        This relies on OS's battle tested scheduling, low runtime overhead and simple.
        But
        Os threads are heavy on memory and resources so we cant use millions at once.

    M:N MODEL- Several third party libraries that offer green threads, where
    many language level tasks M map to a smaller pool of operating system threads N.

    Async/Await - RUST also offers an async system, which lets a single thread pause
    and deal with many waiting tasks like waiting for network respionses wihtout creating a full OS
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
    // and spawns main trheads from 1 to 5 in our main
    // THe output of this can tell us how less overhead we ahve with OS threads
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
even though in our code its clearly the after our regular threads.

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

        handle.join().unwrap();// sequitnal

        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap(); // concurrent
    /*
    JOIN().
    A join handle is an owned value that represrents our background thread.
    If we run .join() we tell the current thread to stop and wait until
    other thread is competely finished.
    This is also calel BLOCKING, when a thread is blocked, its CPU execution time is done

    we use unwrap, because .join retursn a Result.
    Ok(value) the thread completed sucesfully and optionally returneed a value.
    Err(err) the thread panicked while running.
    this is for the backgorund thread, uwnrap handles the result.


    CONCURRENT VS SEQUENTIAL CALL OF .JOIN

    Concurrent - .join at the end()
        the code above, when .join is at the end the main thread spawn
        finishes and when normally it would terminate the whole main()
        and end the background thread spawn, it waits for the backgorund thread
        then program finsihed

    Sequential - .join at the beginning() of main threa dpawn.
        .join at the beginning is stopping the main threa d before it starts
        waits for the backgorund thread spawn to finish then continues its own
        until its goal.
        no actual concurrency occurs, the threads are run in sequence.

     */

    /*
    using MOVE closures with Threads
    Rust rejects borrowed data across threads, and we can use move to fix it.

    Closures default to borroiwng, if they only need to read them
     */

    let v = vec![1,2,3];
    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}") // closure tries to borrow &v
    });
    /*
    because prinln! only reads v, closure borrows v but the compiler throws
    an error, "closures may outlvie the current funciton but it borrows v"
    meaning v might not live while lcosure lives mor ethan v causing a null
    memory pointer.
    Compiler also doenst knwo how long a thread will run, it could run after main() finsihes
    which is also garabge memory.

     */
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}"); // Borrowing v
    });

    drop(v); // Main thread instantly destroys v from memory!

    handle.join().unwrap();

    /* this code is an example where the OS pauses the thread before it runs
    and drops v, conitneus the thread trying to read garabge memory
    now we can fix this with MOVE

    Fix: tell rust tog ive the entrie variable to the new thread instead of
    borrowing it
     */

    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        println!("here's a vector {v:?}")
    });
    handle.join().unwrap();

    /* pushing move infront of the || takes full ownership of v
    v is physically out of the main threads scope and into the spawned threads
    environemtn
    v will stay alive as long the spawned thread exists, and will be cleaned up
    autoamtically after the thread finsihed.

    main() has no access to v, cant drop, or change.
    if both threads need the data we need to .clone() so each thread
    has its own independted copy.
    or use thread safe shared ownerhship types like
    Arc<T> and Mutex<T>

     */



    /* Transfering data between THREADS
    instead of ahving threads try to fight over the same memory, message passing
    allows threads to communiat eby sending pieces of data to each other.
    Go language principle - do not communicate by sharing memory; instead,
    share memory by communicating.

    A channel is like a one-way river:
    Transmitter(tx) - upstream, where we drop data into the river.
    Reciever(rx) - downstream, where we wait to catch the data.
    If either rx or tx is dropped, channel is clsoed.

    MPSC - std::sync::mpsc what does it mean?
    Mpsc stands for Multiple Producer, Single Consumer
    Multiple prodcuers - we can have many threads holding transmitters,
    sending data into the same channel
    Single Consumer - there can only ever be ONE reciving and consuming
    these messages.
     */
    use std::sync::mpsc;
    let (tx,rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("HI");
        tx.send(val).unwrap();
    });

    /* mpsc::channel constructs the channel and returns a tuple (transmitter, reciever)
       move moves tx out of the main thread into spanwed thread whic now owns tx.
       tx.send(val) puts the messsag einto the channel.
       send() takes full ownership of val
       send() also retursn Result<(), SendError<T>> if the receieving end
       rx has already been dropped/destroyed sending fails, because nobody
       to receive it.
       we call .unwrap() if an error occurs to handle it.
       tx automatically gets the .send() because it its attained a transmittion
     */

    let received =rx.recv().unwrap();
    println!("received: {received}");
    /* ABove is printing the received message, there is two ways to grab data
    rx.recv() - Pauses the current thread completely until a message arrives,
    we should use this when the thread has nothing else to do except wait for data.
    rx.try_recv() - checks isntantly, returns OK(message) if one is waiting err if
     When the trhead has other work to do like running a loop or UI, and jus wants to check
     for data periodically.
     this one doesn't block. it just checks.
     */

    let val1 = String::from("Hi");
    tx.send(val).unwrap();
    println!("val is {val}");

    /* ths woudl give a compiler value because in rust sending an object
    down a queue/data stream passes its ownership unlike other languages.
    In other languages THREAD A could modify while THREAD B holds pointer and rememrbs it differently
    rsut solves this by taking ownerhisp of the arugment.
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
    above is sending mutliple values with a simple loop througha vec/
    vec is empty at the end.

    and instead of calling rx.recv().unwrap() everytime we can treat the rx
    as an iterator.
    on each iteraiton of the loop it blocks the thread anwa its for msg.
    if no msg is present loop blcoks until one arrives .
     */






}
