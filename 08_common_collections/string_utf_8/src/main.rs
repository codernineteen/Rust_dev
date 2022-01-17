//strings are implemented as a collection of bytes
//&str :  are stored in the program’s binary and are therefore string slices. - this is only one type
//String type : is provided by Rust’s standard library rather than coded into core lang
//both String and &str are UTF-8 encoded.
fn main() {
    let s1 = "initial contents ".to_string();
    let s2 = String::from("initial contents");
    //Above two codes result is same.

    //#1
    //update string
    //To push more data in string, you can use below methods
    let mut s3 = String::from("foo");
    s3.push_str("bar"); // push_str doesn't take ownership(because it takes slice of string)
    s3.push('c'); // push takes a single character('')

    println!("{}", s3);

    //#2
    //concatenation with '+' operator or format! macro.
    let s4 = s1 + &s2; //s4 takes s1's ownership , but references s2 value.
    println!("{}", s4);
    //we can only add string slice(&str) to a String, we can't do this with two String type.
    //The reason we can use &String here is that compiler can coerce the &String into &str.(deref coercion)
    // e.g) &s2 -> &s2[..]
    //Then why s4 take out of s1's ownership?
    //It's because + operator is another expression of add method of std
    //and add method takes 'self' argument.

    //But if we want to concanate multiple strings, + operator might be tedious way
    //use format! macro.
    let s5 = format!("{},{},{}", s4, s2, s3); //format macro uses references.
    println!("{}", s5);

    //#3
    //3 different ways to look strings : bytes, scalar values, and grapheme clusters.
    // Let's take a hindi word
    //“नमस्ते" -> Vec<u8> -> [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    //“नमस्ते" ->  Unicode scalar -> ['न', 'म', 'स', '्', 'त', 'े']
    //“नमस्ते" -> grapheme clusters -> ["न", "म", "स्", "ते"]
    // computer can choose the interpretation it needs

    //#4
    //Because Rust don't know what type of result you want for indexing
    //you should be specific if you reall want to use indices.
    let hello = "Здравствуйте";
    let s6 = &hello[0..2];
    println!("{}", s6);

    //#5
    //If you want to perform operations on individual Unicode, use .chars() method and iterate it.
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    //if you want bytes of word? use .bytes()
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
