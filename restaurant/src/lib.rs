// practice defining modules with hierarchy representing simple functions a restauraunt may use
mod front_of_house;

pub use crate::front_of_house::hosting;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Apetizer {
        Salad,
        Soup,
    }

    impl Breakfast {
        // without this function, you cannot initialize an instance of "Breakfast" because seasonal_fruit is private.
        // What if it was an optional field? 
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    hosting::add_to_waiting_list();

    // relative path
    hosting::add_to_waiting_list();

    let mut meal = back_of_house::Breakfast::summer("Ryee");
    meal.toast = String::from("Wheat");
    
    // uncommenting next line will not compile because seasonal fruit is private
    // meal.seasonal_fruit = String::from("blueberries");

    let mut apetizer = back_of_house::Apetizer::Salad;
    apetizer = back_of_house::Apetizer::Soup;
}

// idioms
// when bringing in functions into scope. Bring it's parent, this helps differentiate between locally defined functions and imported ones
// when bringing structs, functions and others, it is good practice to bring the entire path into scope. Unless there is a clash of course
// With clashing names of full path imports, the idiom is to rename the import
