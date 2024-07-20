use crate::models::menu::{Menu, MenuItem};
use crate::models::order::Order;
use crate::repositories::menu::MenuRepo;
use crate::repositories::order::OrderRepo;

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
        vec![menu_item1.clone(), menu_item2.clone()],
    );

    let menu_repo = MenuRepo::new();
    menu_repo.add(menu.clone());

    if let Some(retrieve_menu) = menu_repo.get(menu.id()) {
        println!("{:?}", retrieve_menu);
    }

    println!(" ");

    let order_repo = OrderRepo::new();
    let order = Order::new(23, vec![menu_item1.clone(), menu_item2.clone()]);
    order_repo.add(order.clone());
    if let Some(retrieve_order) = order_repo.get_order_by_table_id(order.get_table_id()) {
        println!("{:?}", retrieve_order);
    }
}
