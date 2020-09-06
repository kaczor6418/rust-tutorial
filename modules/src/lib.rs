mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Add utils to list");
        }

        pub fn seat_at_table() {
            println!("Give table number");
        }

        pub mod nested {
            pub fn hello() {
                println!("Hello");
            }
        }
    }

    // mod is private by default so it can not be accessed from parent
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn absolute_and_relative_path() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

use crate::front_of_house::hosting; // we can also add as after use to make an alias for module

fn with_use_keyword() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();
}

use crate::front_of_house::hosting::{add_to_waitlist, seat_at_table, nested::hello}; // we can pull only this methods in which we are interested from module

fn nested_imports() {
    add_to_waitlist();
    seat_at_table();
}

use crate::front_of_house::*; // this will bring every publick member of front_of_house module to our scope
