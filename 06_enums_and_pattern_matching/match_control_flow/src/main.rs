//#1
//Match allows you to compare a value against a seires of patterns
//The power of match is expresiveness of the patterns and the fact that compiler confirms that all possible cases are handled.
#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska,
    // skip
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let penny = Coin::Penny;
    let value_of_penny = value_in_cents(penny);
    let _quarter_alaska = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("{}", value_of_penny);

    let five = Some(5);
    let _six = plus_one(five); //mathoes to Some(i) => Some(i+1)
    let _none = plus_one(None); //matches to None => None
}

fn value_in_cents(coin: Coin) -> u8 {
    //Unlike if, match can return any type.
    //For example, 'if coin {}' <-- this is impossible with if condition.
    match coin {
        //match arms
        //if unmatched , it continues to go down.
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // you can also run multiple line with block.
        //#2
        //patterns that bind to values.
        //In this case, state variable will bind to the value of that quarter's state
        Coin::Quarter(state) => {
            println!("state quarter from {:?}", state);
            25
        }
    }
}

//#3
//Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

//#4
//Be sure to handle every possible case when you work with match expression.

//#5
//catch all patterns and _Placeholder
//If there is a common behavior for each values. you can use other
fn _treat_common(dice_roll: u8) {
    match dice_roll {
        3 => println!("you got hat"),
        7 => println!("you lost hat"),
        other => println!("move player"), //this applied to all dice_roll except 3, 7
    }
}

//When we don't want to use the value in the catch-all pattern. _ works.
fn _continue_rolling(dice_roll: u8) {
    match dice_roll {
        3 => println!("you got hat"),
        7 => println!("you lost hat"),
        _ => (), //this applied to all dice_roll except 3, 7 and nothing happened.
    }
}
