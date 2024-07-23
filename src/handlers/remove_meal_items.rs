use std::sync::{Arc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::json;
use crate::handlers::error::{ErrResp, MESSAGE_ITEMS_REMOVAL_FAILED, MESSAGE_ORDER_NOT_FOUND};
use crate::repositories::order::OrderRepo;

#[derive(Deserialize)]
pub struct RemoveMealItemsReq {
    pub table_id: u32,
    pub meal_item_ids: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RemoveMealItemsResp {
    pub non_removable_meal_item_ids: Vec<Uuid>,
    pub message: String,
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
                error_message: MESSAGE_ORDER_NOT_FOUND.to_string(),
            };
            return Ok(warp::reply::with_status(
                warp::reply::json(&resp),
                StatusCode::NOT_FOUND,
            ));
        }

        if ids.is_empty() {
            Ok(warp::reply::with_status(
                json(&serde_json::json!({})),
                StatusCode::NO_CONTENT,
            ))
        } else {
            let error_resp = RemoveMealItemsResp {
                non_removable_meal_item_ids: ids,
                message: MESSAGE_ITEMS_REMOVAL_FAILED.to_string(),
            };
            Ok(warp::reply::with_status(
                json(&error_resp),
                StatusCode::CONFLICT,
            ))
        }
    }
}