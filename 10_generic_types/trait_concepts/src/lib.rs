//#1
//Trait : functionality which a particular type has and can share with other types.
//For example, we might have NewsArticle struct and Tweet struct
//Both of two structs need a summary
//In this case, we can create Summary trait

//#2
//restriction: we can implement trait on a type only if at least one of the trait or the type is local to our crate.
//For example, std trait 'Display'(external) on a type 'Tweet'(local) (because Tweet is local in this crate)
//Also, we can implement Summary trait(local) to std type Vec<T>(external)
//But, Display trait(external) on Vec<T>(external) is impossible
//This restriction is called 'coherence', 'Orphan rule' (To prevent someone from breaking your code)
use std::fmt::{Debug, Display};

pub trait Summary {
    //A trait can have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon.
    fn summarize_spec(&self) -> String;
    //#4
    //we can call other methods in the same trait(doens't require default implementaion)
    fn summarize_author(&self) -> String;
    //#3
    //you can use default implementaions
    fn summarize_def(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

//Implementing trait on a type(NewsArticle, Tweet)
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
//Implementing trait on a type is quite a similar to implemening  regular methods.
impl Summary for NewsArticle {
    //summarize method has defined in Summary trait
    //Here, we describe summarize in a more specific way for this custom type
    fn summarize_spec(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    //#4
    //we only need to define what summarize_author do
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_spec(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

//#5
//Traits as parameter
//you can take a trait as a parameter and can call any methods of it
//Item can be Tweet or NewsArticle, can't be other types (e.g String, i32)
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize_def());
}

//In more complex cases, we can use trait bounds(previous example is syntax sugar)
//Arguments type should be same because we gave same genercic type T
pub fn notify_bound<T: Summary>(item1: &T, item2: &T) {}

//#6
//we can also specify more than one trait bounds
pub fn notify_mul_bounds<T: Summary + Display>(item: &T) {}

//#7
//Using too many trait bounds can be verbose, we can use 'where' instead
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}
