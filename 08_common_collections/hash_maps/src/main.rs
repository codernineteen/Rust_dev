//HashMap<K, V> , it maps keys of type K to values of type V
use std::collections::HashMap;

fn main() {
    //#1
    //create a new hash map
    let mut scores = HashMap::new(); //generate with new
    scores.insert(String::from("Blue"), 10); //put a value with insert
    scores.insert(String::from("Yellow"), 50);
    //you can create new hash map by using vectors and collect method.
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    //we use underscores, and Rust can infer the types that the hash map contains based on the types of the data in the vectors.
    let mut scoresTwo: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    //#2
    //hash maps take ownership of owned values like String.
    //but just copy for types that implement Copy trait.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    //#3
    //value of hash map -> use get method (remenber 'get' returns Option<&V>)
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // if exists -> Some(&10) / if not exists -> None

    //#4
    //we can iterate over each key value pair of hash map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //#5
    //updating a hash map
    scores.insert(String::from("Blue"), 25); // this overwrite value of existing key 'Blue'
    scores.entry(String::from("Yellow")).or_insert(50); //if key is not existed, insert
    scores.entry(String::from("Blue")).or_insert(50); // nothing happen here.
}
