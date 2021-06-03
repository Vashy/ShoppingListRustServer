pub trait ShoppingList {
    fn add_item(&mut self, item: String);
    fn get_all(&self) -> &Vec<String>;
}

pub struct InMemoryShoppingList {
    store: Vec<String>,
}

impl ShoppingList for InMemoryShoppingList {
    fn add_item(&mut self, item: String) {
        self.store.push(item)
    }

    fn get_all(&self) -> &Vec<String> {
        &self.store
    }
}

impl InMemoryShoppingList {
    pub fn new() -> Self {
        InMemoryShoppingList { store: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::model::InMemoryShoppingList;
    use crate::lib::model::ShoppingList;

    #[test]
    fn should_be_empty_when_created() {
        let shopping_list = InMemoryShoppingList::new();

        assert_eq!(0, shopping_list.get_all().len());
    }

    #[test]
    fn should_have_some_items_when_some_items_are_added() {
        let mut shopping_list = InMemoryShoppingList::new();
        shopping_list.add_item(String::from("Item 1!"));
        shopping_list.add_item(String::from("Item 2!"));

        assert_eq!(2, shopping_list.get_all().len());
        assert_eq!(String::from("Item 1!"), shopping_list.get_all()[0]);
        assert_eq!(String::from("Item 2!"), shopping_list.get_all()[1]);
    }
}
