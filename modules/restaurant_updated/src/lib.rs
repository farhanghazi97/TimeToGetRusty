mod back_of_house;
mod front_of_house;

use back_of_house::meals::Breakfast;

pub use crate::back_of_house::meals;
pub use crate::front_of_house::hosting;
pub use crate::front_of_house::serving;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

pub fn serve_customer() {
    serving::serve_order();
}

pub fn prepare_breakfast() -> Breakfast {
    meals::Breakfast::summer("Wholemeal")
}
