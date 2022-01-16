fn main() {}

//#1
//Define structure
fn _make_structs() {
    //how to create structs
    struct User {
        active: bool, //this type of data is called 'field'
        username: String,
        email: String,
        sign_in_account: bool,
    }
    //how to generate instance from structs
    //declaration order is not important
    //if the instance is mutable, we can change specific field.
    let mut user1 = User {
        active: false,
        email: String::from("example@gmail.com"),
        sign_in_account: false,
        username: String::from("Kimchi"),
    };

    user1.active = true;
    //#2
    //also we can return an instance as last expression in a function
    fn buil_intance(email: String, username: String) -> User {
        User {
            email: email,
            active: true,
            sign_in_account: true,
            username: username,
        }
    }

    //#3
    //To avoid repeating field name and value name, there is a shorthand
    fn build_instance_shorthand(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_account: true,
        }
    }

    //#4
    //Instances from other instances.
    let _user2 = User {
        email: String::from("example2@gmail.com"), //only updated email field from user 1
        ..user1 // This works samely like javascript 'spread' operator.
                //The important thing is this is also data 'move' to prevent double free error.
                //So user1 is no valid anymore
                //If we only copied active, sign_in_contract, user1 would be still valid because those are scalar types and have Copy trait.
    };

    //#5
    //tuple struct
    //There is no named fields
    //you can give whole tuple a name and make the tuple be a different type from other tuples.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0); //color type
    let _origin = Point(0, 0, 0); //point type

    //#6
    //unit struct
    struct AlwaysEqual;
    let _subject = AlwaysEqual;

    //#7
    //The reason why we didn't use &str(string slice) reference.
    //It's because of lifetime concepts . More detail in Chapter 10.
}
