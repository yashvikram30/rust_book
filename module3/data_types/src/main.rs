fn main() {

// Scalar Type:
// A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
// In general, a signed number with n bits can represent numbers between -(2n - 1) and 2n - 1 - 1.

    // floating
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // basic arithematic operations
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1
    let _remainder = 43 % 5;

    // bool
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // char
    let _c = 'z';
    let _z: char = 'Z'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

// Compound Types
// Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    // tuple: a general way of grouping data of different types
    let tup:(i64, u64, f64) = (-2, 500, 3.02 );
    let (_x,_y,_z) = tup; // destructing the tuple
    let _a = tup.0; // we can access the tuple elements like this

    // we can modify individual elements of a mutable tuple
    let mut tup2: (i32, i32) = (1, 2);
    tup2.0 = 0;
    tup2.1 += 5;

    // array: collection of multiple values of same type
    let arr:[i32;5] = [1,2,3,4,5];
    // array stores data in stack memory and not heap memory, and it's size cannot be changed at all
    let _first = arr[0];
    let _second = arr[1];
}


/*



Integer Overflow:

- When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs
- When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping
  In case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on
*/