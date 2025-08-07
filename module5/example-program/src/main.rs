#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        length: 40
    };

    // different output styles using debug trait
    println!("Rect is : {rect1:?}");
    println!("Rect is : {rect1:#?}");

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // this will be printed as well
        height: 50,
    };

    // dbg! macro takes the ownership of the expression, prints it and then returns the ownership
    // therefore, we pass it by refernce because we do not want the ownership of our struct taken away from the main program
    dbg!(&rect2);
}

/*
    Debug Trait:
    The println! macro call will now look like println!("rect1 is {rect1:?}");. 
    Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug. 
    The Debug trait enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code.

    Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. 
    To do that, we add the outer attribute #[derive(Debug)]
*/