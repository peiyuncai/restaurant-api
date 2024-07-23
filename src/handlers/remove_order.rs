use std::sync::{Arc};
use warp::http::{StatusCode};
use crate::handlers::error::{ErrResp, MESSAGE_ORDER_NOT_FOUND, MESSAGE_ORDER_REMOVAL_CONFLICT};
use crate::repositories::order::OrderRepo;

pub struct RemoveOrderHandler {
    order_repo: Arc<OrderRepo>,
}

impl RemoveOrderHandler {
    pub fn new(order_repo: Arc<OrderRepo>) -> Self {
        RemoveOrderHandler {
            order_repo,
        }
    }

    pub fn handle(&self, table_id: u32) -> Result<impl warp::Reply, warp::Rejection> {
        let (result, existed) = self.order_repo.remove_order(table_id);
        if !existed {
            let resp = ErrResp {
                error_message: MESSAGE_ORDER_NOT_FOUND.to_string(),
            };
            return Ok(warp::reply::with_status(
                warp::reply::json(&resp),
                StatusCode::NOT_FOUND,
            ));
        }

        if result {
            Ok(warp::reply::with_status(
                warp::reply::json(&serde_json::json!({})),
                StatusCode::NO_CONTENT,
            ))
        } else {
            let resp = ErrResp {
                error_message: MESSAGE_ORDER_REMOVAL_CONFLICT.to_string(),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&resp),
                StatusCode::CONFLICT,
            ))
        }
    }
}