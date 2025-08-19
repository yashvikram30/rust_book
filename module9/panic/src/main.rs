fn main() {
    // There are two ways to cause a panic in practice: by taking an action that causes our code to panic (such as accessing an array past the end) or by explicitly calling the panic! macro
    // By default, when a panic occurs the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. However, walking back and cleaning up is a lot of work. Rust, therefore, allows you to choose the alternative of immediately aborting, which ends the program without cleaning up.
    
    // method:1
    // panic!("crash and burn");
    
    // method:2 
    // let v = vec![1, 2, 3];
    // v[99];
     
}
