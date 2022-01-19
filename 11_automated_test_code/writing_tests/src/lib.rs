#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    // use rectangle::Rectangle
    use super::*;
    //when success
    #[test] //indicates that this function is a test function
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    //when fail
    #[test] //indicates that this function is a test function
    fn deliberate_failure() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = super::Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = super::Rectangle {
            width: 5,
            height: 1,
        };
        //new macro assert!
        //argument evaluates to a Boolean -> if true, test passed / if false, test failed and panic
        assert!(larger.can_hold(&smaller));
    }
    //assert argument will be true because of '!'
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    //assert_eq test equality , assert_ne test inequality
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }
    #[test]
    fn it_adds_two() {
        //first argument is expected result(left) and second is real result from function(right)
        //If test fails, compiler let us know both of right and left value.
        assert_eq!(4, add_two(2));
    }

    //you can use custom error message with assert macros
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", // panic message
            result                                           //actual result
        );
    }

    //should_panic
    //if test is panic -> test pass (because we know this code should be panic)
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    //Result<T, E> test
    //you can't use should_panic here
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
