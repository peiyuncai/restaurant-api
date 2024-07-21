use std::sync::{Arc, Mutex};
use std::thread::{sleep};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::http::{status, StatusCode};
use crate::libraries::thread_pool::ThreadPool;
use crate::models::meal::MealItemStatus;
use crate::models::menu::MenuItem;
use crate::models::order::Order;
use crate::repositories::order::OrderRepo;

#[derive(Serialize)]
pub struct RemoveOrderResp {
    table_id: u32,
    message: String,
}

pub struct RemoveOrderHandler {
    order_repo: Arc<OrderRepo>,
}

impl RemoveOrderHandler {
    pub fn new(order_repo: Arc<OrderRepo>) -> Self {
        RemoveOrderHandler {
            order_repo,
        }
    }

    pub fn handle(&self, table_id: u32) -> Result<impl warp::Reply, warp::Rejection> { //Result<impl warp::Reply, warp::Rejection>
        let result = self.order_repo.remove_order(table_id);
        if result {
            Ok(warp::reply::with_status(
                warp::reply::json(&"Success"), // You can use a static string for success
                StatusCode::OK,
            ))
        } else {
            let resp = RemoveOrderResp {
                table_id,
                message: "Order cannot be removed as it is already started preparing, completed, or not found".to_string(),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&resp),
                StatusCode::CONFLICT, // or StatusCode::NOT_FOUND depending on your logic
            ))
        }
    }
}