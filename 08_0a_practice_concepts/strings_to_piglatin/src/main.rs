// first -> irst-fay
// apple -> apple-hay

fn main() {
    println!("Hello, world!");
    let first = String::from("first");
}

fn filter_word(word: String) {
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();
    let rest = chars.collect::<String>();
    let result = is_vowel(&first_char);
    if result == true {
        format!("{}-{}", word, "hay")
    } else {
        format!("{}-{}ay", rest, first_char)
    }
}

fn is_vowel(c: &char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
