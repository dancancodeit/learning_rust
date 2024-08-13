// practice defining modules with hierarchy representing simple functions a restauraunt may use
mod front_of_house {
    mod hosting {
        fn add_to_waiting_list() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waiting_list();

    // relative path
    front_of_house::hosting::add_to_waiting_list();
}