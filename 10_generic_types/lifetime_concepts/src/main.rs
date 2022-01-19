//The main aim of lifetimes is to prevent dangling references (which reference a variable is alreay out of scope)
//every reference has a lifetime
//you need to specify lifetime parameters for functions or structs that use references

fn main() {
    //#1
    // lifetime 'a is longer than 'b -> occur dangling error
    //     {
    //         let r;                // ---------+-- 'a
    //                               //          |
    //         {                     //          |
    //             let x = 5;        // -+-- 'b  |
    //             r = &x;           //  |       |
    //         }                     // -+       |
    //                               //          |
    //         println!("r: {}", r); //          |
    //     }                         // ---------+
    //lifetime `a is shorter than `b -> no error

    {
        let x = 5; //lifetime 'b start
        let r = &x; //lifetime 'a start
        println!("r: {}", r);
    } //both lifetime end

    //#2
    //lifetime annotation syntax
    //use apostrophe(') and lowercase(conventionally it is 'a )
    //&i32        - a reference
    //&'a i32     - a reference with an explicit lifetime
    //&'a mut i32 - a mutable reference with an explicit lifetime

    //#3
    //generic lifetimes in functions
    //result lifetime should be smaller of the lifetimes of x and y
    //but keep in mind that it depends on the way you declare function signature
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyz"; //static str doesn't have a problem, but if you use String type here, it doesn't live long enough.
        result = longest(string1.as_str(), string2);
    }
    println!("The longest string is {}", result);

    //#4
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'"); //this reference owned by novel variable
    let i = ImportantExcerpt {
        part: first_sentence,
    }; // i goes out of scope earlier than novel variable. So this is valid.
}

//#3
//Lifetime Annotations in Function Signatures
//lifetime of parameters and lifetime of returned references are same -> returned reference will be valid as long as both params are.
//The lifetime annotations become part of the contract of the function
//the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//#4
//lifetime annotations in Struct definition
struct ImportantExcerpt<'a> {
    part: &'a str, // this annotation means that an instance of this struct can't outlive the reference it holds in its field.
}

//#5 - lifetime elision rules.
//Lifetimes on function or method parameters are called input lifetimes,
//and lifetimes on return values are called output lifetimes.
//Three rules to figure out what lifetimes references have when there aren’t explicit annotations.

//1.each parameter that is a reference gets its own lifetime parameter .
//e.g ) fn foo<'a>(x: &'a i32); / fn foo<'a, 'b>(x: &'a i32, y: &'b i32); / and so on.
//2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
//e.g ) fn foo<'a>(x: &'a i32) -> &'a i32.
//3. if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method,
//the lifetime of self is assigned to all output lifetime parameters.

//Let's apply this rule with example
//fn first_word(s: &str) -> &str {
// apply first rule ->
//fn first_word<'a>(s: &'a str) -> &str {
// apply second rule ->
//fn first_word<'a>(s: &'a str) -> &'a str {
// after applying rule -> we can see all the parameters have lifetime and also returned reference has lifetime -> no need to apply lifetime on ourselves

//Another example
//fn longest(x: &str, y: &str) -> &str {
//first rule ->
//fn longest<'a,'b>(x: &'a str, y: &'b str) -> &str {
//second, third rule can't be applied -> return doesn't have lifetime
//In this case, we should specify lifetime on ourselves.

//#6
//All string literals have the 'static lifetime -> can lice for the entire duration of program
