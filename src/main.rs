use std::sync::Arc;
use std::time::Duration;
use serde::Deserialize;
use uuid::Uuid;
use warp::{Filter, Reply};
use crate::handlers::add_meal_items::{AddMealItemsHandler, AddMealItemsReq};
use crate::handlers::add_order::{AddOrderHandler, AddOrderReq};
use crate::handlers::query_meal_item::QueryMealItemHandler;
use crate::handlers::query_order::QueryOrderHandler;
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

    if let Some(retrieve_menu) = menu_repo.get(menu.id()) {
        println!("{:?}", retrieve_menu.lock().unwrap());
    }

    println!(" ");

    let order_repo = Arc::new(OrderRepo::new());

    // let order = Order::new(23, vec![menu_item1.clone(), menu_item2.clone()]);
    // order_repo.add(order.clone());

    let pool = ThreadPool::new(2);
    let add_order_handler = Arc::new(AddOrderHandler::new(order_repo.clone(), pool.clone()));
    let query_order_handler = Arc::new(QueryOrderHandler::new(order_repo.clone(), pool.clone()));
    let query_meal_item_handler = Arc::new(QueryMealItemHandler::new(order_repo.clone()));
    let add_meal_items_handler = Arc::new(AddMealItemsHandler::new(order_repo.clone(), pool.clone()));
    // let result = add_order_handler.handle(AddOrderReq {
    //     table_id: 32,
    //     menu_items: vec![menu_item1.clone(), menu_item2.clone()],
    // });
    // match result {
    //     Ok(reply) => {
    //         // To print the Reply, convert it to a string using warp::http::Response
    //         let response = reply.into_response();
    //         let body = response.into_body();
    //         // Assuming the body is utf-8, otherwise use appropriate handling
    //         if let Ok(body_str) = hyper::body::to_bytes(body).await {
    //             println!("Success: {}", String::from_utf8_lossy(&body_str));
    //         } else {
    //             println!("Success but unable to read body");
    //         }
    //     }
    //     Err(rejection) => {
    //         // Handle the rejection case
    //         println!("Error: {:?}", rejection);
    //     }
    // }
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

    let routes = add_order
        .or(query_order)
        .or(query_meal_item)
        .or(add_meal_items);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    // let order_repo_arc = Arc::clone(&order_repo);
    // if let Some(order_arc) = order_repo_arc.get_order_by_table_id(order.get_table_id()) {
    //     let order = order_arc.lock().unwrap();
    //     println!("{:?}", order);
    //     for meal_item_arc in order.get_meal_items() {
    //         let meal_item = meal_item_arc.lock().unwrap();
    //         let meal_item_id = meal_item.id();
    //         let table_id = order.get_table_id();
    //         let order_repo_arc = Arc::clone(&order_repo_arc);
    //         pool.execute(move || {
    //             if let Some(meal_item_arc) = order_repo_arc.get_order_meal_item(table_id, meal_item_id) {
    //                 let meal_item = meal_item_arc.lock().unwrap();
    //                 if meal_item.is_removed() { return; }
    //
    //                 let cooking_time_in_min = meal_item.cooking_time_in_min();
    //                 drop(meal_item);
    //
    //                 println!("start preparing {}", meal_item_id);
    //                 order_repo_arc.update_order_meal_item_status(table_id, meal_item_id, MealItemStatus::Preparing);
    //
    //                 sleep(Duration::from_secs(cooking_time_in_min as u64));
    //
    //                 order_repo_arc.update_order_meal_item_status(table_id, meal_item_id, MealItemStatus::Completed);
    //                 println!("completed {}", meal_item_id);
    //             }
    //         })
    //     }
    // }

    tokio::time::sleep(Duration::from_secs(10)).await;

    if let Some(order_arc) = order_repo.get_order_by_table_id(32) {
        let order = order_arc.lock().unwrap();
        println!("{:?}", order);
    }
    pool.lock().unwrap().join();
}
