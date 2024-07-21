use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use warp::Reply;
use crate::libraries::thread_pool::ThreadPool;
use crate::models::meal::MealItemStatus;
use crate::models::menu::MenuItem;
use crate::models::order::{Order, OrderStatus};
use crate::repositories::order::OrderRepo;

#[derive(Serialize)]
pub struct MealItemResp {
    meal_item_id: Uuid,
    name: String,
    price: String,
}

#[derive(Serialize)]
pub struct OrderResp {
    remaining_cooking_time_upper_bound_in_min: u32,
    total_price: String,
    status: OrderStatus,
    meal_items: Vec<MealItemResp>,
}

pub fn convertPrice(price: f64) -> String {
    let price_in_cents: u64 = (price * 100.0) as u64;
    price_in_cents.to_string()
}

impl OrderResp {
    pub fn new(order: Order) -> Self {
        let mut order_resp = OrderResp {
            total_price: "".to_string(),
            remaining_cooking_time_upper_bound_in_min: 0,
            status: OrderStatus::Received,
            meal_items: vec![],
        };

        order_resp.total_price = convertPrice(order.get_total_price());

        let mut has_preparing = false;
        let mut has_received = false;
        let mut has_completed = true;
        let mut all_removed = true;

        for item_arc in order.get_meal_items().iter() {
            let item = item_arc.lock().unwrap();
            if item.is_removed() { continue; }

            all_removed = false;

            let item_resp = MealItemResp {
                meal_item_id: item.id(),
                name: item.get_name(),
                price: convertPrice(item.price()),
            };
            order_resp.meal_items.push(item_resp);

            match item.get_status() {
                MealItemStatus::Received => {
                    has_received = true;
                    has_completed = false;
                    order_resp.remaining_cooking_time_upper_bound_in_min += item.cooking_time_in_min();
                }
                MealItemStatus::Preparing => {
                    has_preparing = true;
                    has_completed = false;
                    order_resp.remaining_cooking_time_upper_bound_in_min += item.cooking_time_in_min();
                }
                MealItemStatus::Completed => {}
            }
        }

        if all_removed {
            order_resp.status = OrderStatus::Canceled;
        } else if has_preparing {
            order_resp.status = OrderStatus::Preparing;
        } else if has_received {
            order_resp.status = OrderStatus::Received;
        } else if has_completed {
            order_resp.status = OrderStatus::Completed;
        }

        order_resp
    }
}

#[derive(Serialize)]
pub struct QueryOrderResp {
    order: OrderResp,
}

pub struct QueryOrderHandler {
    order_repo: Arc<OrderRepo>,
    thread_pool: Arc<Mutex<ThreadPool>>,
}

impl QueryOrderHandler {
    pub fn new(order_repo: Arc<OrderRepo>, thread_pool: Arc<Mutex<ThreadPool>>) -> Self {
        QueryOrderHandler {
            order_repo,
            thread_pool,
        }
    }

    pub fn handle(&self, table_id: u32) -> Result<impl warp::Reply, warp::Rejection> {
        if let Some(order_arc) = self.order_repo.get_order_by_table_id(table_id) {
            let order = order_arc.lock().unwrap().clone();
            let resp = QueryOrderResp {
                order: OrderResp::new(order),
            };
            Ok(warp::reply::json(&resp))
        } else {
            Err(warp::reject::not_found())
        }
    }
}