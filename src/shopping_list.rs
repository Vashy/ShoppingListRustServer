use std::sync::Mutex;

use rocket::{State, http::Status};
use rocket_contrib::json::Json;

use crate::lib::model::{InMemoryShoppingList, ShoppingList};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct ShoppingListItem {
    description: String,
}

impl ShoppingListItem {
    fn new(description: String) -> Self { Self { description } }
}

#[post("/shopping-list/items", data = "<body>")]
pub fn add_item(state: State<Mutex<InMemoryShoppingList>>, body: Json<ShoppingListItem>) -> Status {
    state.lock().unwrap().add_item(body.description.clone());

    Status::Created
}

#[get("/shopping-list/items")]
pub fn get_items(state: State<Mutex<InMemoryShoppingList>>) -> Json<Vec<ShoppingListItem>> {
    let lock = state.lock().unwrap();
    let body = lock
        .get_all()
        .iter()
        .map(|description| ShoppingListItem::new(description.to_string()))
        .collect();

    Json(body)
}
