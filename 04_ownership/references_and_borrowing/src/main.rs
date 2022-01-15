//References is like a pointer -> points address data stored -> we can access it
//Unlike a pointer, it points to a valid value of a particular type.

fn main() {
    reference_value();
    borrowing_value();
    mutable_references();

    //#4
    //combining mutable and immutable references
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem -> multiple immutable ref allowed.
                 //let r3 = &mut s;  BIG PROBLEM
                 //we can't have a mutable ref while we have immutable one.
    println!("{} and {}", r1, r2); // r1, r2 scope ended after they used.
    let r3 = &mut s; // This is not problem because r1, r2 scope ended.
    println!("{}", r3);
}

//#1
fn reference_value() {
    let s1 = String::from("hello");
    //ampersand(&) means references
    //this refers to value without taking ownership;
    let len = calculate_length(&s1); // we put references of s1, not own it.

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // we take reference as &String
    s.len()
} //nothing happens because it doesn't own anything, only refers to.
  //Creating references is called borrowing.

//#2
//you can't change anything from borrowed variable
fn borrowing_value() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    // some_string.push_str(", world"); <- this cause error.
    println!("{}", some_string);
}

//#3
//mutable references.
fn mutable_references() {
    let mut s = String::from("hello"); // declare variable as mutable

    change_ref(&mut s); // give ref with mut keyword
}

fn change_ref(some_string: &mut String) {
    // give ref with mut keyword
    some_string.push_str(", world");
}
// you can't mutate more than one ref at the same time
// let r1 = &mut s;
// let r2 = &mut s;
// above code is not possible -> It prevents data race.

//#5
//dangling ref - create a dangling pointer which refers to a location in memory that may have already been given to someone else.

//Rust compiler doesn't allow this.
//fn get_dangle() {
//    let reference_to_nothing = dangle(); // return ref. This is impossible because s is dropped out of scope.
//}
//fn dangle() -> &String {
//   let s = String::from("hello"); -> declare String

//   &s return String reference
//} s is dropped
