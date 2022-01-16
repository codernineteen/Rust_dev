//if let syntax lets you combine if and let into less verbose way to handle values.

fn main() {
    //This is what we do with match expressoin
    let config_max_one = Some(3u8);
    match config_max_one {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    //This is if let syntax
    //it works samely as above match expression
    //But you lose exhaustive checking which you can do with match expression.
    let config_max_two = Some(3u8);
    if let Some(max) = config_max_two {
        println!("The maximum is configured to be {}", max);
    }

    //Like you do with normal conditions, you can also use else here.
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

    fn match_coins(coin: Coin) {
        let mut count = 0;
        if let Coin::Quarter(state) = coin {
            println!("state existed")
        } else {
            count += 1;
        }
    }
}
