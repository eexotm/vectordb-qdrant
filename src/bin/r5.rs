//! ============================================================================
//! The Rust Programming Language — Chapter 8: Common Collections
//! https://doc.rust-lang.org/book/ch08-00-common-collections.html
//!
//!   8.1   Storing Lists of Values with Vectors
//!   8.2   Storing UTF-8 Encoded Text with Strings
//!   8.3   Storing Keys with Associated Values in Hash Maps
//! ============================================================================

fn main() {

    let v: Vec<i32> = Vec::new();
    // Basic declaration of a fresh empty vector.
    let v = vec![1,2,3,4];
    // vec! is a macro, it automatically infers the type

    let mut v = Vec::new();
    v.push(2); // simple way to push a value onto an vector
    v.push(6);
    v.push(3);
    // v.push(String::from("Hello")); //fails because we declared v as a <i32>

    let third = &v[2];
    println!("third elemnt:{}", third); // without getting ownership, we can read the third elemnt
    // if [index] is not within v.len() program !panic s so we Can use get()
    let third = v.get(4);
    match third {
        Some(t) => println!("The third elemnt is {t}"),
        None => println!("Failed to fetch the index.")
    }
    let v = vec![100,32,56];
    for i in &v {
        println!("{i}");
    }
    //simple way to iterate over items ina vectors we can also iterate mutably
    let mut v = vec![100,32,45];
    for i in &mut v {
        *i += 50;
    } // we dereference the operator with * and add 50 to it

    // STORING MULTIPLE TYPES IN A VECTOR
    // we can use enums to do so

    enum spreadsheet {
        int(i32),
        float(f64),
        text(String),}

    let row =
        vec![spreadsheet::int(23),
             spreadsheet::float(23.32),
             spreadsheet::text(String::from("Hello"))];
    // here we have a type spreadsheet vector which includes differnet types. breaking no rules


    // DEFINING STRINGS
    let data = "content";
    let  s = data.to_string();
    //will turn the &str variable "data" to a String.

    // STRING OPERATIOSN
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{s}"); // will output hello world
    // push_str does not take owernship
    // push only pushes a single character
    s.push('!');
    // we can alsom combine two strings with ownership transfer control
    let s1 = String::from("Cyber");
    let s2 = String::from("punk");
    let s3 = s1 + &s2; // s1 is moved but s2 is borrowed.
    println!("{s3}"); // outputs Cyberpunk. + uses the add method.
    // when using + gets too complicated like trying to create tic-tac-toe from the three strings.
    // we can just use format!
    // Format! is like println! but instead of printing to the screen it returns a String with the contents.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    /*
    Slicing a string:
    we cant slice strings in Rust like the other languages.
    A string is a wrapper over a Vec<u8>
    the string "hola" has len 4, with each letters being 1 byte
    but the string "Здравствуйте" is ode snot start with three but the
    Cyrillic letter Ze so thast two bytes.
    Therefore a strings cant be interperted with their unicode scalar value.
    you could technically try to slice things with a range like
    let s = &hello[0..4] where hello is the russian string.
    we can get 3 but its risky as we if we slice 0..1 rust will panic.

     */
    // ITERATING over strings.
    for c in "Зд".chars(){
        println!("{c}");
        // will print 3 д
    }
    // we can also use .bytes to iterate as well.

    // HASH MAPS
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("yellow"),50);
    // HASHMAPS are value-key structures/ lsits
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // Here, score will have the value thats associated with team_name which is blue/
    // the get method retursn Option<&V>, we just handle it by an unwrap_or
    // we call copied because without it, .get() would return Option<&i32> which we cant counterract with 0.
    // it has to be borrowed value, but if we copy it we change the return to Option<i32>.

    let mut scores = HashMap::new();
    scores.insert("A", 10);
    scores.insert("B",20);
    for (key, value) in scores
    {
        println!("{key} : {value}");
    }
    // let score = scores.get("A").copied().unwrap();
    // example where this would fail because loop consumes scores.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // we can simply override a hashmap pair by restating it.

    // ENTRY api for hashmaps

    let mut guns = HashMap::new();

    guns.insert(String::from("Pistol"), 10);

    guns.entry(String::from("Pistol")).or_insert(10);
    // check if "pistol" has an value, if ot enter 10.
    guns.entry(String::from("SMG")).or_insert(50);
    // checks if SMG is there, if not it enters both SMG and its key
    println!("{guns:?}");

    let text = "hello world wonderful world hello";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    // if its the first time we see that word , we enter 0. if not we increment it.
}


