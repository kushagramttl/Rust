// all items (functions, methods, structs, enums, modules, and constants) are private by default
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn create_waitlist() {
    add_to_waitlist();
    add_to_waitlist();
}