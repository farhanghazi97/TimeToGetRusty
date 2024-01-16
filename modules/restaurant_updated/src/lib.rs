mod services;

pub use crate::services::back_of_house::meals;
pub use crate::services::front_of_house::hosting;
pub use crate::services::front_of_house::serving;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

pub fn serve_customer() {
    serving::serve_order();
}

pub fn prepare_breakfast() -> meals::Breakfast {
    meals::Breakfast::summer("Wholemeal")
}
