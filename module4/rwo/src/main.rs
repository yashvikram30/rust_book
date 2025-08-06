fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    println!("Third element is {}", *num);
    v[2] = 4;
    v.push(4);

    /*
        All variables can read, own, and (optionally) write their data.
        Creating a reference will transfer permissions from the borrowed place to the reference.
        Permissions are returned once the reference’s lifetime has ended.
        Data must outlive all references that point to it.
     */
}

// if a value does not own heap data, then it can be copied without a move.