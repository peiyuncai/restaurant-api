use std::env;
use std::str::FromStr;
use std::sync::Arc;
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
    println!("Hello, Welcome to our restaurant!");

    let args: Vec<String> = env::args().collect();

    let mut pool_size: usize = 2; // Default value
    if args.len() == 2 {
        match args[1].parse::<usize>() {
            Ok(size) => pool_size = size,
            Err(_) => {
                eprintln!("Invalid thread pool size: {}", args[1]);
                std::process::exit(1);
            }
        }
    }

    let order_repo = Arc::new(OrderRepo::new());
    let pool = ThreadPool::new(pool_size);
    let add_order_handler = Arc::new(AddOrderHandler::new(order_repo.clone(), pool.clone()));
    let query_order_handler = Arc::new(QueryOrderHandler::new(order_repo.clone()));
    let remove_order_handler = Arc::new(RemoveOrderHandler::new(order_repo.clone()));
    let add_meal_items_handler = Arc::new(AddMealItemsHandler::new(order_repo.clone(), pool.clone()));
    let query_meal_item_handler = Arc::new(QueryMealItemHandler::new(order_repo.clone()));
    let remove_meal_items_handler = Arc::new(RemoveMealItemsHandler::new(order_repo.clone()));

    let add_order = warp::post()
        .and(warp::path("orders"))
        .and(warp::body::json())
        .and_then(move |req: AddOrderReq| {
            let handler = add_order_handler.clone();
            async move { handler.handle(req) }
        });

    let query_order = warp::get()
        .and(warp::path("orders"))
        .and(warp::path::param())
        .and(warp::query::<QueryOrderParams>()) // Query parameter
        .and_then(move |table_id: u32, params: QueryOrderParams| {
            let handler = query_order_handler.clone();
            async move { handler.handle(table_id, params.include_removed_items) }
        });


    let query_meal_item = warp::get()
        .and(warp::path("meal-items"))
        .and(warp::path::param())
        .and(warp::path::param())
        .and_then(move |table_id: u32, meal_item_id: Uuid| {
            let handler = query_meal_item_handler.clone();
            async move { handler.handle(table_id, meal_item_id) }
        });

    let add_meal_items = warp::post()
        .and(warp::path("meal-items"))
        .and(warp::body::json())
        .and_then(move |req: AddMealItemsReq| {
            let handler = add_meal_items_handler.clone();
            async move { handler.handle(req) }
        });

    let remove_meal_items = warp::delete()
        .and(warp::path("meal-items"))
        .and(warp::body::json())
        .and_then(move |req: RemoveMealItemsReq| {
            let handler = remove_meal_items_handler.clone();
            async move { handler.handle(req) }
        });

    let remove_order = warp::delete()
        .and(warp::path("orders"))
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
