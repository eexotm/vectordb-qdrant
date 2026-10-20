//! ============================================================================
//! The Rust Programming Language — Chapter 10: Generic Types, Traits, and Lifetimes
//! https://doc.rust-lang.org/book/ch10-00-generics.html
//!
//!   10.1  Generic Data Types
//!   10.2  Defining Shared Behavior with Traits
//!   10.3  Validating References with Lifetimes
//! ============================================================================

fn main() {

    // Generics are used to "generalize" and impelmnt multipel data types into a single funcito/


    fn largest<T:std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    struct Point<T> {
        x:T,
        y:T,
    }

    let integer = Point{x:5, y:3}; // Mismatch types cuz all types should be T which we declared as an integer by saying x:5
    // if we do this however

    struct Point2<T,R>{
        x:T,
        y:R,
    }

    //simple enough now we can do two differnet types on both.

    //T is generic for the receiving data types, it can be anything but it cant be differnet thigns at the same time.

    /*
    OPTION enum is similar to this, it has <T> on Some(T) because Some should be able to reutn ranything
    Result on the other hand is T,E. for Ok(T) and Err(E) ebcause those are differnet .

     */


    // Generic types on implementiation of method son structs/enums

    impl<T> Point<T>{
        fn x(&self) -> &T {
            &self.x
        }
    } // method x, will return itself, by using generics so it oculd be anything.
    // if we didnt use generics we would mismatch the genriic type we would havefrom Point.
    // &self is a borrow and borrow return.


    let p3 = Point(x:5, y: 4 );

    let px = p3.x();
    println!("px: {}", px) // prints 5

























}



