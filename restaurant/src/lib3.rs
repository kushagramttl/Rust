/**
 * 
 * The example shows how we can make use of a structure inside a module
 * It can be seen that the method 'summer' and variable 'toast' are accessible outside the module
 * 
 */

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast = String::from(toast),
                seasonal_fruit = String::from("peach")
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}