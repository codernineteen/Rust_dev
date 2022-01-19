//import library - refer to Cargo.toml
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("kimchiman"),
        content: String::from("new tweet upcoming"),
        reply: false,
        retweet: false,
    };
    //default trait implementaion
    println!("Here is new tweet tweet!!. {}", tweet.summarize_def());

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_clone(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_clone(&char_list);
    println!("The largest char is {}", result);
}

//Let's fix lergest function example in generic_concepts
// '>' (larger) operator is defined as a default method on the std trait (std::cmp::PartialOrd)
// i32, char has Copy trait -> generic function let params have types in that -> so we can't move out the value out of list[0]
// So we need to add Copy trait to the bounds of T
fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list.iter() {
        if *item > largest {
            largest = item.clone();
        }
    }

    largest
}
