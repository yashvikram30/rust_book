const TWO: u32 = 1 + 1;
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


    // Shadowing:  the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends
    // difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}")
}
// Difference between immutable variables and const: you aren’t allowed to use mut with constants
// also, variables cannot be declared in global scope, unlike constants