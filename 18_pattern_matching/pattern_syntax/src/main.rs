fn main() {
    //#1
    //matching literal
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        //treat all other execptions
        _ => println!("other number"),
    }
    //#2
    //mathcing named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        //we are in the new scope inside the match expression,
        //this is new variable y which binded Some, not y(10) outside of this scope
        //Therefore, this y will match any value in Some()
        Some(y) => println!("Matched, y = {:?}", y),
        //can't reach here
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    //#3
    //multiple patterns
    let x = 2;

    match x {
        // '|' means 'or'
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    //#4
    //matching ranges of values with ..=
    let x = 5;

    match x {
        //1,2,3,4,5 will matched = 1|2|3|4|5
        //ranges only allowed with numerica values or char values
        1..=5 => println!("one through five"),
        //'a'..='j' => println!("this is an alphabet between a and j"),
        _ => println!("something else"),
    }

    //#5
    //destructuring
    let p = Point { x: 0, y: 7 };
    //define x, y by destructuring
    let Point { x, y } = p;
    //we also can destructure literal values as part of the struct pattern
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    assert_eq!(0, x);
    assert_eq!(7, y);

    //destrucuring enum
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        //destructure x, y
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        //destructure text
        Message::Write(text) => println!("Text message: {}", text),
        //destruturing nested structs and enums
        //destructure nested enum and then destructure r,g,b
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
    }

    //complex destructuring
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    //#6
    //wildcard pattern
    //this function ignore first argument
    //we can use underscore syntax not only for match expression, but also function parameter
    //underscore with function parameter is useful when you make a trait, but one of trait implementaion doesn't need the parameter
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    //we can ignore parts of a value with a nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        //ignore values already existed
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        //add new value
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    //ignoring an unused variable by staring its name with _
    let _x = "dummy"; //this variable will not cause compile warning

    //#7
    //ignoring remaining parts of a value with ..
    struct PointThreeD {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = PointThreeD { x: 0, y: 0, z: 0 };

    match origin {
        //ignore rest of points
        //PointThreeD {x , y: _, z: _} => println!("x is {}",x ) -> this works samely
        PointThreeD { x, .. } => println!("x is {}", x),
    }

    //.. syntax will expand to as many values as it needs to be
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    //if you write code like below, it cause error because it is ambiguous
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     },
    // }

    //#8
    //match guard
    //match guard is an additional if condition specified after the pattern in a match arm that must also match
    let num = Some(4);

    match num {
        //addtional condition
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    //match guard can solve pattern shadowing(same name of variables, but works differently because of scope)
    let x = Some(5);
    let y = 5;

    match x {
        Some(50) => println!("Got 50"),
        //now we can use y in match expression scope
        //because if n == y is not a pattern
        //this is outer y rather than a new-shadowed y
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    //#9
    //@ Bindings
    let msg = Message::Move { x: 5, y: 5 };

    match msg {
        Message::Move {
            //destructure id field and bind its value to id_variable , then check range condition
            x: x_variable @ 3..=7,
            ..
        } => println!("Found an x in range: {}", x_variable), //so we can use id_variable here
        Message::Move { x: 10..=12, .. } => {
            println!("Found an x in another range")
        }
        Message::Move { x, .. } => println!("Found some other x: {}", x),
        _ => println!("ignore other patterns"),
    }
}
struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,                    //we can't destructure Quit
    Move { x: i32, y: i32 }, //struct-like enum
    Write(String),           //tuple-like enum
    ChangeColor(Color),      //tuple-like enum + another enum nested
}
