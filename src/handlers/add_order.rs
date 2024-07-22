use std::sync::{Arc, Mutex};
use std::thread::{sleep};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use warp::http::StatusCode;
use crate::handlers::add_meal_items::{ErrResp, MenuItemReq};
use crate::handlers::query_order::{OrderResp};
use crate::libraries::thread_pool::ThreadPool;
use crate::models::meal::{MealItemStatus};
use crate::models::menu::MenuItem;
use crate::models::order::Order;
use crate::repositories::order::OrderRepo;

#[derive(Deserialize)]
pub struct AddOrderReq {
    table_id: u32,
    menu_items: Vec<MenuItemReq>,
}

#[derive(Serialize)]
pub struct AddOrderResp {
    order: OrderResp,
}

pub struct AddOrderHandler {
    order_repo: Arc<OrderRepo>,
    thread_pool: Arc<Mutex<ThreadPool>>,
}

impl AddOrderHandler {
    pub fn new(order_repo: Arc<OrderRepo>, thread_pool: Arc<Mutex<ThreadPool>>) -> Self {
        AddOrderHandler {
            order_repo,
            thread_pool,
        }
    }

    pub fn handle(&self, req: AddOrderReq) -> Result<impl warp::Reply, warp::Rejection> {
        let mut menu_items = Vec::with_capacity(req.menu_items.len());
        for menu_item_req in req.menu_items {
            let menu_item = MenuItem::create(
                menu_item_req.menu_item_id,
                menu_item_req.name,
                menu_item_req.price,
            );
            menu_items.push(menu_item);
        }

        let order = Order::new(req.table_id, menu_items.clone());
        self.order_repo.add(order.clone());
        for meal_item_arc in order.get_meal_items() {
            let meal_item = meal_item_arc.lock().unwrap();
            let meal_item_id = meal_item.id();
            let table_id = req.table_id;
            let order_repo_arc = Arc::clone(&self.order_repo);

            self.thread_pool.lock().unwrap().execute(move || {
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

        if let Some(order) = self.order_repo.get_order_by_table_id(req.table_id) {
            let resp = AddOrderResp {
                order: OrderResp::new(order.lock().unwrap().clone(), false),
            };
            return Ok(warp::reply::with_status(
                warp::reply::json(&resp),
                StatusCode::OK,
            ));
        }

        let resp = ErrResp {
            message: StatusCode::INTERNAL_SERVER_ERROR.to_string()
        };
        Ok(warp::reply::with_status(
            warp::reply::json(&resp),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}