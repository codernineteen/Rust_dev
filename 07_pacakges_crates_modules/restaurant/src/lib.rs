//#1
//command : cargo new --lib restaurant
//both src/main.rs and src/lib.rs are crates, main.rs is binary and lib.rs is library.
//you can regard this as front-end part
//mod is how to define a module.

//#2
//if we want to call a function in module, we should know 'paths'
// two form of path.
// An absolute path starts from a crate root by using a crate name or a literal crate.
// A relative path starts from the current module and uses self, super, or an identifier in the current module.
// call a function == know path
mod front_of_house {
    //#3
    //The way privacy works in Rust is that all items (functions, methods, structs, enums, modules, and constants) are private by default.
    //parents can't use private of children, but child modules can do
    //but you can expose these privates with 'pub' keyword
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
//#3
//relative paths with super
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        super::serve_order(); //super go to the parent module of this back_of_house
                              //In this case, crate itself. (crate::back_of_house:: ...)
    }

    //#3
    //making structs and enums public
    pub struct Breakfast {
        //breads can be changed
        pub toast: String,
        //seasonal fruit is private
        seasonal_fruit: String,
    }
    //Enums aren’t very useful unless their variants are public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

//#2
pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

//module tree structure
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment
// front_of_house is parent of hosting and serving . hosting and serving are children of front_of_hosue.
