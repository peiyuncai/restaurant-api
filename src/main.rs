use crate::models::menu::{Menu, MenuItem};
use crate::repositories::menu::MenuRepo;

mod models;
mod repositories;

fn main() {
    println!("Hello, world!");

    let menu_item1 = MenuItem::new(
        String::from("Burger"),
        8.55,
    );
    let menu_item2 = MenuItem::new(
        String::from("Fries"),
        3.49,
    );
    let menu = Menu::new(
        String::from("fast food menu"),
        vec![menu_item1, menu_item2],
    );

    let menu_repo = MenuRepo::new();
    menu_repo.add(menu.clone());

    if let Some(retrieve_menu) = menu_repo.get(menu.id()) {
        println!("{:?}", retrieve_menu);
    }

}
