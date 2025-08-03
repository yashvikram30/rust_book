fn main() {
    
    let num: u8 = 6;

    if num % 2 ==0 { // the condition must be a bool (true or false,and not even 0 or 1) otherwise, the code will return an error
        println!("even")
    } else if num % 3 ==0 {
        println!("not even & divisible by 3")
    } else {
        println!("divisible by neither")
    }

    // Using if in a let Statement
    let condition = false;
    let num = if condition {5} else {6}; // here the values to be returned must be of the same type
    println!("num is : {num}");
}
