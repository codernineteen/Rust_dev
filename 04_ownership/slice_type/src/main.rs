fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);
    let word_slice = slice_first_word(&s);

    println!("{}", word);
    println!("{}", word_slice)
    //word is 5, but we can't do anything with 5.
}

//#1
//without slice
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // as_bytes method convert string to an array of bytes

    //iter returns each element in a collection and
    //enumerate wraps iter result and returns each element as part of tuple.
    //(i, &item) destructures enumerate result.
    //we gave String as a reference, so we get enumerate result as a reference. That's why we use &item.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

//#2
//with string slice : a reference to part fo a String.
fn _slice_string(s: &String) {
    // First number is starting index, last number is ending index
    let _slice = &s[0..2];
    // if you start zero, there is no need 0 with starting point
    let _slice_zero = &s[..2];
    // if you want to slice til last, it is similar
    let _slice_last = &s[0..];
    // start zero, end last
    let _slice_zero_last = &s[..];
}

//#3
//rewrite previous function to get first word
fn slice_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

//#4
//you can also slice array
