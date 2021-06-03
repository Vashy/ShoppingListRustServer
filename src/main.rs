#![feature(proc_macro_hygiene, decl_macro)]

pub mod lib;
pub mod shopping_list;
pub mod errors_catchers;

#[macro_use]
extern crate rocket;

use std::sync::Mutex;

use rocket::Rocket;

use crate::lib::model::InMemoryShoppingList;

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/api", routes![shopping_list::get_items, shopping_list::add_item, shopping_list::delete_items])
        .register(catchers!(errors_catchers::not_found, errors_catchers::unprocessable_entity))
        .manage(Mutex::new(InMemoryShoppingList::new()))
}

fn main() {
    rocket().launch();
}
