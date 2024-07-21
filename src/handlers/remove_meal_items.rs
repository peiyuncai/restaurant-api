use std::sync::{Arc, Mutex};
use std::thread::{sleep};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::json;
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
    pub reason: String,
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
        let resp = RemoveMealItemsResp {
            table_id: req.table_id,
            non_removable_meal_item_ids: ids,
            reason: "selected meal items are already started preparing or completed".to_string(),
        };
        Ok(warp::reply::json(&resp))
    }
}