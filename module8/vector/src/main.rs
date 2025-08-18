fn main() {

    // standard method to create vectors, Vec<T> represents the type of the vector
    let mut v: Vec<i32> = Vec::new();
    v.push(32);
    println!("{:#?}",v);
    // we can shadow the vector to delete it's mutability now but it's a overkill
    
    let v2 = vec![1,2,3];
    println!("{:#?}",v2);

    // two methods of reading from a vector
    let i = &v2[1];  // this method will lead to the program crashing if the index does not exist
    println!("The first index is: {}", i);

    let j = v2.get(2); // no crashing takes place here
    match j {
        Some(val) => println!("The val of index 2 is: {}", val),
        None => println!("There is no index 1")
    }

    /*
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        v.push(6);
        println!("The first element is: {first}"); // the rust compiler panics, because we do not have write access to a variable, in the lifetime of a immutable reference
     */

    // iterating over a vector
    for i in &v2 {
        print!("{:?} ",i);
    }

    let mut v3 = vec![10,20,30];
    for i in &mut v3 {
        *i = *i + 10;
    }
    println!("\n{:?}", v3);

    /*
        Important Note: non copyable types cannot be moved out of the vector by indexing (such as string cannot be moved by using v[0], it must be referenced as &v[0]).
     */

    // using enums to store elements of different types in a vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // we can match here to use this vector 

}
