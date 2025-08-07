// defining a struct
struct User{
    username: String,
    active: bool,
    email : String,
    sign_in_count: u64,
}

// a function defined to create struct instances by just inputting the params
fn build_user(username: String, email: String) -> User {

    User {
        username,
        active: true,
        email,
        sign_in_count: 1
    }
}

fn main() {
    
    // creating a mutable instance of the struct
    let mut user1 = User{
        username: String::from("100xYash"),
        active: false,
        email: String::from("yashvikram8250@gmail.com"),
        sign_in_count: 1
    };

    // accessing the values in the struct
    println!("{}",user1.username);

    // updating the struct
    user1.username = String::from("yash.sol");

    // creating a new struct instance from the old struct instance
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("yashvikram6@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };

    // alter syntax for the same thing
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    /*
        Here's what Rust does field by field:
        -  user2.email: Gets the new String we provided.
        -  user2.active: Gets a copy of user1.active (since bool is a Copy type). user1.active is still fine.
        -  user2.sign_in_count: Gets a copy of user1.sign_in_count (since u64 is a Copy type). user1.sign_in_count is still fine.
        -  user2.username: Because we didn't specify a new username, it takes the one from user1. Since String is a Move type, the ownership of the username data is moved from user1 to user2.

        The Result: user1 has now given away ownership of its username. Because a part of the user1 struct is now invalid, the compiler forbids you from using the entire user1 variable anymore to prevent accidental use of moved data.
     */
}
