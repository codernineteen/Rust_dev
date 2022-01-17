mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// With use keyword, now hosting is valid in this scope which use 'use' keyword.
// 1.absolute path
use crate::front_of_house::hosting;

// 2.relative path
//use self::front_of_house::hosting;

// 3. with as keyword.
use crate::front_of_house::hosting as custom_hosting;

// 4. re-exporting
// pub crate::front_of_house::hosting;

// 5. external pacakge
use rand::Rng;

// 6. nested paths to avoid taking up a vertical spaces
use std::io::{self, Write};
use std::{cmp::Ordering, collections}; //different librarys, but have common part // from same library, but different children module.

// 7. glob operator (with asterisk)
use std::collections::*;

//hosting is now valid name in this scope
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
