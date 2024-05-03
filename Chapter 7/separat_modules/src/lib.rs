mod front_of_house;

pub use crate::front_of_house::{hosting, posting};

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    posting::let_me_post();
}
