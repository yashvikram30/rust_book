use std::collections::HashMap;

fn main() {
    
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 32);
    scores.insert(String::from("red"), 40);

    let data = match scores.get("red") { // getting values from a hashmap using get, can also use .unwrap_or()
        Some(value) => value,
        None => &-1
    };

    println!("\nThe value is: {}\n",data);

    // For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!


    // updating value in hashmap

    // method:1 using insert
    scores.insert(String::from("green"), 10);
    scores.insert(String::from("green"), 25);

    // method:2 adding a key only if it isn't present
    scores.entry(String::from("yellow")).or_insert(50);
    scores.entry(String::from("red")).or_insert(60);

    println!("{:#?}",scores);

    // method:3 updating value based on old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}",map);


}
