// we can define the another_function before or after main, it doesn't matter
// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value.

// If you add a semicolon to the end of an expression, you turn it into a statement
fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    // let x = (let y = 6); // returns an error because let y = 6 is a statement and not expression

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}"); // this will compile as the block {} returns the value 4 to y

    let p = five();
    println!("The value of x is: {p}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Functions with return value
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// fn plus_one(x: i32) -> i32 { // this function will not return any value, and therefore show error, because of the ;
//     x + 1;
// }