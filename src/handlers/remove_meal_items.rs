use std::sync::{Arc, Mutex};
use std::thread::{sleep};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::json;
use crate::handlers::remove_order::RemoveOrderResp;
use crate::libraries::thread_pool::ThreadPool;
use crate::models::meal::{MealItem, MealItemStatus};
use crate::models::menu::MenuItem;
use crate::models::order::Order;
use crate::repositories::order::OrderRepo;

#[derive(Deserialize)]
pub struct RemoveMealItemsReq {
    pub table_id: u32,
    pub meal_item_ids: Vec<Uuid>,
}

#[derive(Serialize)]
pub struct RemoveMealItemsResp {
    pub table_id: u32,
    pub non_removable_meal_item_ids: Vec<Uuid>,
    pub message: String,
}

pub struct RemoveMealItemsHandler {
    order_repo: Arc<OrderRepo>,
    thread_pool: Arc<Mutex<ThreadPool>>,
}

impl RemoveMealItemsHandler {
    pub fn new(order_repo: Arc<OrderRepo>, thread_pool: Arc<Mutex<ThreadPool>>) -> Self {
        RemoveMealItemsHandler {
            order_repo,
            thread_pool,
        }
    }

    pub fn handle(&self, req: RemoveMealItemsReq) -> Result<impl warp::Reply, warp::Rejection> { //Result<impl warp::Reply, warp::Rejection>
        let ids = self.order_repo.remove_order_meal_items(req.table_id, req.meal_item_ids);
        if ids.is_empty() {
            let success_resp = RemoveMealItemsResp {
                table_id: req.table_id,
                non_removable_meal_item_ids: ids,
                message: "All specified items have been removed successfully.".to_string(),

            };
            Ok(warp::reply::with_status(
                warp::reply::json(&success_resp),
                StatusCode::OK,
            ))
        } else {
            let error_resp = RemoveMealItemsResp {
                table_id: req.table_id,
                non_removable_meal_item_ids: ids,
                message: "Some items could not be removed as they are already started preparing, completed, or not found.".to_string(),
            };
            Ok(warp::reply::with_status(
                json(&error_resp),
                StatusCode::CONFLICT,
            ))
        }
    }
}