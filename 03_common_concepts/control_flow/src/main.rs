fn main() {
    let number = 0;

    //#1
    //type 1
    if number < 7 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //#2
    //type 2 (boolean type) - rust doens't convert type of variable for condtion
    if number != 0 {
        println!("number is existed");
    }
    
    //#3
    //expression
    let result = is_number_zero(number);
    println!("{}", result);

    //#4
    //if in a statement;
    //This is similar to javascript ternary expression
    //Of course, you can't mismatch type.
    let pending_number: i32 = if result { 0 } else { -1 };
    println!("{}", pending_number);


    //#5
    example_loop();
    //#6
    example_while();
    //#7
    example_for();
    example_for_nicer_way();
    example_for_nicer_way_reverse();
}

//#3
//if, else if , else
//Caution : if you use 'else if' too much, it is not readable.
fn is_number_zero(number: i32) -> bool { 
    if number == 0 {
        true
    } else if number > 0 {
        false
    } else {
        false
    }
}

//#5
//loop - infinite, you can break loop with 'break' and skip current iteration for next iteration'continue'
//you can nest loop, if you wanna give two different break point with nested loop, you can give label for one.
//you can return loop to add value next to break
fn example_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("{}", count);
        let mut remain = 10;

        loop {
            println!("{}", remain);
            if remain == 9 {
                break ;
            }
            if count == 10 {
                break 'counting_up;
            }
            remain -= 1;
        }
        count += 1;
    }
}

//#6
//conditional loop -while
fn example_while() {
    let mut count = 0;
    while count < 5 {
        println!("count is not 5");
        count += 1;
    }
    println!("count is 5");
}

//#7
//loop through a collection with 'for'
// for is strong when you iterate array.
fn example_for() {
    let arr = [1,2,3,4,5];
    for element in arr {
        println!("{}", element);
    }
}

fn example_for_nicer_way() {
    for element in 1..5 {
        println!("{}", element);
    }
}

fn example_for_nicer_way_reverse() {
    for element in (1..5).rev() {
        println!("{}", element);
    }
}