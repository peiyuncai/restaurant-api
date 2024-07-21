use std::sync::Arc;
use std::time::Duration;
use serde::Deserialize;
use uuid::Uuid;
use warp::{Filter};
use crate::handlers::add_meal_items::{AddMealItemsHandler, AddMealItemsReq};
use crate::handlers::add_order::{AddOrderHandler, AddOrderReq};
use crate::handlers::query_meal_item::QueryMealItemHandler;
use crate::handlers::query_order::QueryOrderHandler;
use crate::handlers::remove_meal_items::{RemoveMealItemsHandler, RemoveMealItemsReq};
use crate::handlers::remove_order::{RemoveOrderHandler};
use crate::libraries::thread_pool::ThreadPool;
use crate::models::menu::{Menu, MenuItem};
use crate::repositories::menu::MenuRepo;
use crate::repositories::order::OrderRepo;

mod models;
mod repositories;
mod libraries;
mod handlers;

#[derive(Deserialize)]
struct QueryOrderParams {
    include_removed_items: bool,
}

#[tokio::main]
async fn main() {
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

    let order_repo = Arc::new(OrderRepo::new());
    let pool = ThreadPool::new(2);
    let add_order_handler = Arc::new(AddOrderHandler::new(order_repo.clone(), pool.clone()));
    let query_order_handler = Arc::new(QueryOrderHandler::new(order_repo.clone()));
    let remove_order_handler = Arc::new(RemoveOrderHandler::new(order_repo.clone()));
    let add_meal_items_handler = Arc::new(AddMealItemsHandler::new(order_repo.clone(), pool.clone()));
    let query_meal_item_handler = Arc::new(QueryMealItemHandler::new(order_repo.clone()));
    let remove_meal_items_handler = Arc::new(RemoveMealItemsHandler::new(order_repo.clone()));

    let add_order = warp::post()
        .and(warp::path("add_order"))
        .and(warp::body::json())
        .and_then(move |req: AddOrderReq| {
            let handler = add_order_handler.clone();
            async move { handler.handle(req) }
        });

    let query_order = warp::get()
        .and(warp::path("query_order"))
        .and(warp::path::param())
        .and(warp::query::<QueryOrderParams>()) // Query parameter
        .and_then(move |table_id: u32, params: QueryOrderParams| {
            let handler = query_order_handler.clone();
            async move { handler.handle(table_id, params.include_removed_items) }
        });


    let query_meal_item = warp::get()
        .and(warp::path("query_meal_item"))
        .and(warp::path::param())
        .and(warp::path::param())
        .and_then(move |table_id: u32, meal_item_id: Uuid| {
            let handler = query_meal_item_handler.clone();
            async move { handler.handle(table_id, meal_item_id) }
        });

    let add_meal_items = warp::post()
        .and(warp::path("add_meal_items"))
        .and(warp::body::json())
        .and_then(move |req: AddMealItemsReq| {
            let handler = add_meal_items_handler.clone();
            async move { handler.handle(req) }
        });

    let remove_meal_items = warp::post()
        .and(warp::path("remove_meal_items"))
        .and(warp::body::json())
        .and_then(move |req: RemoveMealItemsReq| {
            let handler = remove_meal_items_handler.clone();
            async move { handler.handle(req) }
        });

    let remove_order = warp::post()
        .and(warp::path("remove_order"))
        .and(warp::path::param())
        .and_then(move |table_id: u32| {
            let handler = remove_order_handler.clone();
            async move { handler.handle(table_id) }
        });

    let routes = add_order
        .or(query_order)
        .or(add_meal_items)
        .or(query_meal_item)
        .or(remove_meal_items)
        .or(remove_order);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    pool.lock().unwrap().join();
}
