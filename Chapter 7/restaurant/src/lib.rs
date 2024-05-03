mod back_of_house {

    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        // hosting::add_to_waitlist(); // to use this i need use on this scope
        super::deliver_order();
        super::front_of_house::hosting::seat_at_table();
    }

    fn cook_order() {}
}

fn deliver_order() {}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod customer {
    use crate::front_of_house::hosting::add_to_waitlist;

    pub fn eat_at_restaurant() {
        add_to_waitlist();
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();

    // Realtive path
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Wheat");

    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
