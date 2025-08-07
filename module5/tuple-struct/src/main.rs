// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit struct
struct AlwaysEqual;


fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    //Unlike tuples, tuple structs require you to name the type of the struct when you destructure them. For example, we would write let Point(x, y, z) = origin; to destructure the values in the origin point into variables named x, y, and z.

    let subject = AlwaysEqual;
    //Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
}

// tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a . followed by the index to access an individual value.