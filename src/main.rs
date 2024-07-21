use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use crate::libraries::thread_pool::ThreadPool;
use crate::models::meal::MealItemStatus;
use crate::models::menu::{Menu, MenuItem};
use crate::models::order::Order;
use crate::repositories::menu::MenuRepo;
use crate::repositories::order::OrderRepo;

mod models;
mod repositories;
mod libraries;
mod handlers;

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
        println!("{:?}", retrieve_menu.lock().unwrap());
    }

    println!(" ");

    let order_repo = Arc::new(OrderRepo::new());

    let order = Order::new(23, vec![menu_item1.clone(), menu_item2.clone()]);
    order_repo.add(order.clone());
    let mut pool = ThreadPool::new(2);


    let order_repo_arc = Arc::clone(&order_repo);
    if let Some(order_arc) = order_repo_arc.get_order_by_table_id(order.get_table_id()) {
        let order = order_arc.lock().unwrap();
        println!("{:?}", order);
        for meal_item_arc in order.get_meal_items() {
            let meal_item = meal_item_arc.lock().unwrap();
            let meal_item_id = meal_item.id();
            let table_id = order.get_table_id();
            let order_repo_arc = Arc::clone(&order_repo_arc);
            pool.execute(move || {
                if let Some(meal_item_arc) = order_repo_arc.get_order_meal_item(table_id, meal_item_id) {
                    let meal_item = meal_item_arc.lock().unwrap();
                    if meal_item.is_removed() { return; }

                    let cooking_time_in_min = meal_item.cooking_time_in_min();
                    drop(meal_item);

                    println!("start preparing {}", meal_item_id);
                    order_repo_arc.update_order_meal_item_status(table_id, meal_item_id, MealItemStatus::Preparing);

                    sleep(Duration::from_secs(cooking_time_in_min as u64));

                    order_repo_arc.update_order_meal_item_status(table_id, meal_item_id, MealItemStatus::Completed);
                    println!("completed {}", meal_item_id);

                }
            })
        }
    }

    sleep(Duration::from_secs(10));
    if let Some(order_arc) = order_repo.get_order_by_table_id(order.get_table_id()) {
        let order = order_arc.lock().unwrap();
        println!("{:?}", order);
    }
    pool.join();
}
