// The type HashMap<K, V> stores a mapping of keys
// of type K to values of type V using a hashing
// function, which determines how it places these
// keys and values into memory.

// Hash maps are useful when you want to look up
// data not by using an index, as you can with
// vectors, but by using a key that can be of
// any type.

use std::collections::HashMap;

fn main() {
    // Creating a hash map
    let mut scores = HashMap::new();

    // Inserting (key, value) pairs into hash map
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing values in a hash map by key
    let team_name = String::from("Blue");
    // .copied() maps a Option<&T> to Option<T>
    // .unwrap_or(0) gets the value from the "Some" variant of
    // "Option" enum. If there is no value, it uses the provided
    // default (in this case, 0)
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The {team_name} team score is: {score}");

    // Iterating a hash map
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Rust's ownership principles work the same way for hash
    // maps. For types that implement the "Copy" trait, like i32,
    // the values are copied into the hash map (that is, the hash
    // map does not take ownership). For owned types like "String",
    // the values will be moved and the hash map will be the owner
    // of those values. Let's take a look:

    let field_name_1 = String::from("Favourite Colour");
    let field_value_1 = String::from("Blue");
    let mut map_1 = HashMap::new();
    map_1.insert(field_name_1, field_value_1);

    // Notice how the compiler is throwing an error for the println!()
    // This makes sense - "field_name" and "field_value" were moved
    // into the hash map (and thus ownership has been transferred).
    // Thus, these variables are now invalid.

    // println!("{field_name_1}: {field_value_1}");

    // If we insert references to values into the hash map, the values
    // won't be moved and thus ownership is not transferred. The values
    // that the references point to must be valid for at least as long
    // as the hash map is valid.
    let mut map_2 = HashMap::new();
    let field_name_2 = "Favourite Team";
    let field_value_2 = "Arsenal FC";
    map_2.insert(field_name_2, field_value_2);

    // We can still use "field_name_2" and "field_value_2"
    // here!
    println!("{field_name_2}: {field_value_2}");

    // --------------- UPDATING A HASH MAP ---------------
    // Given a key already has some value assigned, updating
    // a hash map could involve:
    //    -> replacing the old value with the new value, completely
    //       disregarding the old value
    //    -> keep the old value and ignore the new value
    //    -> combine the old value and the new value in some way
    // Given a key does not have a value assigned, updating a
    // hash map could involve:
    //    -> only then adding the new value

    // Let's take a look at examples for each use case!

    // --------------- OVERWRITING A VALUE ---------------

    let mut map_3 = HashMap::new();
    map_3.insert(String::from("Green"), 10);
    map_3.insert(String::from("Green"), 25);
    // Observe the println!() below, notice how the original
    // value of 10 was overwritten by the latest value!
    println!("{:?}", map_3);

    // --- ADDING A (KEY, VALUE) PAIR ONLY IF KEY ISN'T PRESENT ---

    // .entry() checks if the given key exists in the hash map
    // .or_insert() checks the return value of entry():
    //    -> if the key exists, it ignores the update
    //    -> if the key does not exist, it inserts the provided
    //       value for that key
    map_3.entry(String::from("Red")).or_insert(50);
    map_3.entry(String::from("Green")).or_insert(100);
    println!("{:?}", map_3);

    // ---------- UPDATING A VALUE BASED ON OLD VALUE ----------
    let text = "hello world wonderful world";
    let mut map_4 = HashMap::new();
    for word in text.split_whitespace() {
        let count = map_4.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map_4);
}
