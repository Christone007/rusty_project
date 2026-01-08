mod front_of_house;
mod back_of_house;

use crate::front_of_house::hosting;
use crate::back_of_house::Breakfast;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);
}
