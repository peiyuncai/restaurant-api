use std::sync::{Arc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::json;
use crate::handlers::add_meal_items::{ErrResp, MESSAGE_ITEM_NOT_FOUND};
use crate::repositories::order::OrderRepo;

#[derive(Deserialize)]
pub struct RemoveMealItemsReq {
    table_id: u32,
    meal_item_ids: Vec<Uuid>,
}

#[derive(Serialize)]
pub struct RemoveMealItemsResp {
    table_id: u32,
    non_removable_meal_item_ids: Vec<Uuid>,
    message: String,
}

pub struct RemoveMealItemsHandler {
    order_repo: Arc<OrderRepo>,
}

impl RemoveMealItemsHandler {
    pub fn new(order_repo: Arc<OrderRepo>) -> Self {
        RemoveMealItemsHandler {
            order_repo,
        }
    }

    pub fn handle(&self, req: RemoveMealItemsReq) -> Result<impl warp::Reply, warp::Rejection> {
        let (ids, existed) = self.order_repo.remove_order_meal_items(req.table_id, req.meal_item_ids);
        if !existed {
            let resp = ErrResp {
                message: MESSAGE_ITEM_NOT_FOUND.to_string(),
            };
            return Ok(warp::reply::with_status(
                warp::reply::json(&resp),
                StatusCode::NOT_FOUND,
            ));
        }

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
                message: "Some items could not be removed as they are already started preparing, or completed.".to_string(),
            };
            Ok(warp::reply::with_status(
                json(&error_resp),
                StatusCode::CONFLICT,
            ))
        }
    }
}