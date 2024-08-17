// practice defining modules with hierarchy representing simple functions a restauraunt may use
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waiting_list() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

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
    crate::front_of_house::hosting::add_to_waiting_list();

    // relative path
    front_of_house::hosting::add_to_waiting_list();

    let mut meal = back_of_house::Breakfast::summer("Ryee");
    meal.toast = String::from("Wheat");
    
    // uncommenting next line will not compile because seasonal fruit is private
    // meal.seasonal_fruit = String::from("blueberries");

    let apetizer = back_of_house::Apetizer::Salad;
    
}
