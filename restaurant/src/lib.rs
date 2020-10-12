/**
 *
 * Based on the Rust book module available at:
 * https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html
 * The crates can be imported to the module by using the 'use' keyword
 * If there are multiple modules that are to be imported form the same mod then they can be imported in the following way:
 * use std::io::{self, Write};
 * If all the public items defined in the path need to be imported, the the * operator can be used.
 *
 */
// all items (functions, methods, structs, enums, modules, and constants) are private by default
// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }
mod front_of_house;
pub use crate::front_of_house::hosting::add_to_waitlist;

/**
 * The import can be combined with the pub keyowrd which would look like:
 * pub use crate::front_of_house::hosting;
 * The default scope of import is private to the scope
 * However, when the public keyword is used, the import is public
 */

pub fn create_waitlist() {
    add_to_waitlist();
    add_to_waitlist();
}
