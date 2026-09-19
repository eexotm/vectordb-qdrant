//! ============================================================================
//! The Rust Programming Language — Chapter 9: Error Handling
//! https://doc.rust-lang.org/book/ch09-00-error-handling.html
//!
//!   9.1   Unrecoverable Errors with `panic!`
//!   9.2   Recoverable Errors with `Result`
//!   9.3   To `panic!` or Not to `panic!`
//! ============================================================================

use std::arch::x86_64::_mm_test_mix_ones_zeros;
use std::io::ErrorKind;

fn main() {
    /*
    Error handling in rust is the result of the compiler needing us to
    calculate every possibility of an action before it compiles.
    We give more effort in developing in exchange for comfort in after deployement.
    There is two major groups of errors, Unrecoverable, Recovrable.
    Unrecoverable - symptomps of bugs, things we need the program stopped.
    Recoverable - things we can redirect for a proper result, no ened to crash the program.
     */

    // UNRECOVERABLE ERRORS WITH PANIC!
    // Rust will panic, unwind, clean the stack.
    /* UNWINDING - RUST walks back up teh stack and cleans up data from each function
    we cna chooose to jsut abort and not clean as well, but that means the OS is responsible for the memory cleanup
     */

    // panic!("doom and despair"); // panic macro syntax

    // most of the time panic is called without us explciltiy using it.

    let v = vec![1,2,3,4];
    v[99]; // panic will called here and the output will show us which line and function caused it.

    /* In C accessing beyond a list's length gives undefined behaviour.
    which means you could get an element in that data structure even though
    your list h as nothing to do with it, which can lead to vulnerabilites,
    corruption. Rust stops this.
     */


    // Recoverable errors with RESULT

    enum Result<T,E> {
        Ok(T),
        Err(E),
    }
    // T and E are generic type parameters. T represent the OK variant E represents teh Error variant

    use std::fs::File;
    let file_result = File::open("hello.txt");
    // the return type of the function File::open is a Result<T,E>.
    // T has been filled in by the type of sucess value which is the file handle
    // E is the std::io:Error, which is the fail to open file error.
    // This return type simply means the function might succeed or
    // fail in such a way that does not require an panic
    // as not accessing a file is not the end of the world

    // as this line stands, the file_result will have the insatnce of Ok
    // which contains the file handle, if it fails file_result be an instance of
    // Err that contains more information about where th error occured.

  //  let file= match &file_result{
     //   Ok(f) => f,
       // Err(error) => panic!("Problem opening file, {error:?}"),
   //  };
    // if no file named hello.txt exists we panic.
    // Though there are easier ways to do this if our error option is to just panic

    // Dealing with different type of error cases
    use std::io::ErrorKind;
    let file = match file_result {
        Ok(n) => n,
        Err(e) => match e.kind() { // matching error type
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(n) => n,
                Err(er) => panic!("File does not exist, attempt to create failed."),

            }
            _ => {
                panic!("Problem opening the file {e:?}");
            }
        },
    };
    /* IN here we can see that our second match bracket does not use Ok and Err
    this is becasue the Err variant in our initial match is io::error
    which means we can match it with known structurual variables like
    we did with the ErrorKind::NotFound variant.
    the use of  _=> {} means "any other case not covered"
    though again, since we resort to panicing on all erorr cases eventually
    these could be rewritten with unwrap variants easily
     */

    // here is the more clean, efficent version

    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|e| {
                panic!("Problem creating the file: {e:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // unwrap_or_else returns Ok(t) directly if closure never runs
    // if Err(e) calls our closure with our set argument

    //result.unwrap();                    // Ok(t) -> t | Err(e) -> panic (generic message)
    // result.expect("msg");               // Ok(t) -> t | Err(e) -> panic (msg + e)
    // result.unwrap_or(default);          // Ok(t) -> t | Err(_) -> default (default is eagerly evaluated NOW, even if Ok)
    // result.unwrap_or_else(|e| { ... }); // Ok(t) -> t | Err(e) -> closure(e) (lazy: closure only runs on Err)


    //Propagating the Error
    // the concept of basically passing any error to the caller rather
    //than handling it ourself.
    use std::io::{self, Read};
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_from_file = File::open("hello.txt");
        let mut username_final = match username_from_file {
            Ok(t) => t,
            Err(e) => return Err(e),
        };
        let mut username = String::new();
        match username_final.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

}
