use std::sync::Arc;
use warp::reply::Reply;
use warp::hyper::body::to_bytes;
use warp::http::StatusCode;
use crate::usecases::models::error::{ErrResp, MESSAGE_ITEMS_PARTIALLY_REMOVED, MESSAGE_ORDER_NOT_FOUND};
use crate::usecases::handlers::remove_meal_items::{RemoveMealItemsHandler, RemoveMealItemsReq, RemoveMealItemsResp};
use crate::models::meal::{MealItem, MealItemStatus};
use crate::models::menu::MenuItem;
use crate::models::order::Order;
use crate::repositories::order::OrderRepo;

#[tokio::test]
async fn test_remove_meal_items_handler_handle_success() {
    let order_repo = Arc::new(OrderRepo::new());

    let handler = RemoveMealItemsHandler::new(order_repo.clone());

    let menu_item = MenuItem::new(String::from("fries"), String::from("345"));
    let meal_item = MealItem::create(menu_item);
    let mut order = Order::new(1, vec![]);
    order.add_meal_items(vec![meal_item.clone()]);
    order_repo.add(order);

    let request = RemoveMealItemsReq {
        table_id: 1,
        meal_item_ids: vec![meal_item.id()],
    };
    let response = handler.handle(request).unwrap();

    let response = response.into_response();

    let status = response.status();
    assert_eq!(status, StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn test_remove_meal_items_handler_handle_not_found() {
    let order_repo = Arc::new(OrderRepo::new());

    let handler = RemoveMealItemsHandler::new(order_repo.clone());

    let request = RemoveMealItemsReq {
        table_id: 1,
        meal_item_ids: vec![],
    };
    let response = handler.handle(request).unwrap();

    let response = response.into_response();

    let status = response.status();
    let body = to_bytes(response.into_body()).await.unwrap();
    let body_bytes = body.to_vec();
    let actual_body: ErrResp = serde_json::from_slice(&*body_bytes).expect("failed to parse");

    let expected_body: ErrResp = ErrResp { error_message: MESSAGE_ORDER_NOT_FOUND.to_string() };

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(expected_body, actual_body);
}

#[tokio::test]
async fn test_remove_meal_items_handler_handle_partial_success() {
    let order_repo = Arc::new(OrderRepo::new());

    let handler = RemoveMealItemsHandler::new(order_repo.clone());

    let menu_item = MenuItem::new(String::from("fries"), String::from("345"));
    let mut meal_item = MealItem::create(menu_item);
    meal_item.update_state(MealItemStatus::Preparing);
    let mut order = Order::new(1, vec![]);
    order.add_meal_items(vec![meal_item.clone()]);
    order_repo.add(order);

    let request = RemoveMealItemsReq {
        table_id: 1,
        meal_item_ids: vec![meal_item.id()],
    };

    let response = handler.handle(request).unwrap();

    let response = response.into_response();

    let status = response.status();
    let body = to_bytes(response.into_body()).await.unwrap();
    let body_bytes = body.to_vec();
    let actual_body: RemoveMealItemsResp = serde_json::from_slice(&*body_bytes).expect("failed to parse");

    let expected_body = RemoveMealItemsResp {
        non_removable_meal_item_ids: vec![meal_item.id()],
        message: MESSAGE_ITEMS_PARTIALLY_REMOVED.to_string(),
    };

    assert_eq!(status, StatusCode::OK);
    assert_eq!(expected_body, actual_body);
}
