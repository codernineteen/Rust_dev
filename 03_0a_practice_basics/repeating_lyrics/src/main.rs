fn main() {
    let lyrics = [
        "Two turtle-doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "I sent 11 pipers piping",
        "12 drummers drumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "nineth",
        "tenth", "11th", "12th",
    ];

    for day in days {
        println!("On the {} day of Christmas\nMy true love sent to me", day);
        let mut index = 0;
        if day == "first" {
            continue;
        } else if day == "second" {
            println!("{}", lyrics[index]);
        } else if day == "third" {
            index = 1;
            for i in 0..index {
                println!("{}", lyrics[i]);
            }
        } else if day == "fourth" {
            index = 2;
            for i in 0..index {
                println!("{}", lyrics[i]);
            }
        } else if day == "fifth" {
            index = 3;
            for i in 0..index {
                println!("{}", lyrics[i]);
            }
        } else if day == "sixth" {
            index = 4;
            for i in 0..index {
                println!("{}", lyrics[i]);
            }
        } else if day == "seventh" {
            index = 5;
            for i in 0..index {
                println!("{}", lyrics[i]);
            }
        } else if day == "eigth" {
            index = 6;
            for i in 0..index {
                println!("{}", lyrics[i]);
            }
        } else if day == "nineth" {
            index = 7;
            for i in 0..index {
                println!("{}", lyrics[i]);
            }
        } else if day == "tenth" {
            index = 8;
            for i in 0..index {
                println!("{}", lyrics[i]);
            }
        } else if day == "11th" {
            index = 9;
            for i in 0..index {
                println!("{}", lyrics[i]);
            }
        } else {
            index = 10;
            for i in 0..index {
                println!("{}", lyrics[i]);
            }
        }
        println!("And a partridge in a pear tree\n");
    }
}
