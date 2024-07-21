use std::sync::{Arc, Mutex};
use std::thread::{sleep};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use crate::libraries::thread_pool::ThreadPool;
use crate::models::meal::MealItemStatus;
use crate::models::menu::MenuItem;
use crate::models::order::Order;
use crate::repositories::order::OrderRepo;

#[derive(Deserialize)]
pub struct AddOrderReq {
    pub table_id: u32,
    pub menu_items: Vec<MenuItem>,
}

#[derive(Serialize)]
pub struct AddOrderResp {
    table_id: u32,
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

    pub fn handle(&self, req: AddOrderReq) -> Result<impl warp::Reply, warp::Rejection> { //Result<impl warp::Reply, warp::Rejection>
        let order = Order::new(req.table_id, req.menu_items);
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

        let resp = AddOrderResp {
            table_id: req.table_id,
        };
        Ok(warp::reply::json(&resp))
    }
}