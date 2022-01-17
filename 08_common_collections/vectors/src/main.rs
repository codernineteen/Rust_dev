// Vec<T> known as vector.
// vector can only store values of the same type.
fn main() {
    //#1
    //declaration vector
    let v1: Vec<i32> = Vec::new(); //Here, type annotation needed, vector are implemented using generics
                                   //#2
                                   //with initial value
    let v2 = vec![1, 2, 3]; //This is a vector macro, no need type annotation

    //#3
    //update vector
    let mut v3 = Vec::new(); //To change value, need mut keyword
    v3.push(5); //To update, can use push method
                //We don't need type annotation because Rust infer this push value type.

    //#4
    //dropping
    {
        // -> invalid
        let v4 = vec![1, 2, 3, 4]; // -> can use vector
    } // -> v4 goes out of scope and is freed here.

    //#5
    //read elements of vector
    let v5 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v5[2];
    println!("{}", third);

    match v5.get(2) {
        Some(third) => println!("{}", third), // Here third is not related to above third
        None => println!("there is no third element"),
    }

    //let does_not_existing = &v5[100]; -> this cause program to be panic
    let does_not_existing = v5.get(100); // this returns 'None' without panic

    //#6
    //you cannot borrow vector as mutable for immutable variable
    //vector originally allocated memory
    //if you add a new element -> re allocating for new memory
    //you immutable variable may points to wrong address (deallocated memory)

    //#7
    //iterating
    for i in &v5 {
        println!("{}", i);
    }

    //#8
    //To store mutltiple types into a vector
    //we can use enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    } //SpreadsheetCell type

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
        //These are SpreadsheetCell type, but different value inside.
    ];
}
