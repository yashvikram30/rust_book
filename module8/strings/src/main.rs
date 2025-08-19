fn main() {
    
    // methods of creating strings
    let s1 = String::new();
    let mut s2 = String::from("Hello World");

    let s3 = "heya, how are you"; //string slice
    let s4 = s3.to_string(); // converts &str to String type

    let s5 = String::from("This is another way to create strings!");

    // we can update the string using .push_str, which can add a string literal, and .push which can add a char
    s2.push_str(". I am yash"); // if we use a parameter/ variable, .push_str DOES NOT TAKE THE OWNERSHIP
    s2.push('.');

    println!("{:#?}",s2);

    /*
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    */

    let s6 = format!("{s2} {s3}"); // formatting allows concatenation without changing the ownership of s1
    println!("{}",s6);
    
    // Rust strings don’t support indexing.
    // let s1 = String::from("hi");
    // let h = s1[0]; // this will return an error

    // however, we can slice a string like this
    let hello = "Здравствуйте";
    let slice = &hello[0..4]; 
    // You should use caution when creating string slices with ranges, because doing so can crash your program. this is due to complications in the string

    // printing bytes representation
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // printing char representation
    for c in "Зд".chars() {
        println!("{c}");
    }

}
