use std::sync::Arc;
use uuid::Uuid;
use warp::reply::Reply;
use warp::hyper::body::to_bytes;
use warp::http::StatusCode;
use crate::usecases::models::error::{ErrResp, MESSAGE_ITEM_NOT_FOUND};
use crate::usecases::handlers::query_meal_item::{MealItemResp, QueryMealItemHandler, QueryMealItemResp};
use crate::models::meal::MealItem;
use crate::models::menu::MenuItem;
use crate::models::order::Order;
use crate::repositories::order::OrderRepo;

#[tokio::test]
async fn test_query_meal_item_handler_handle_success() {
    let order_repo = Arc::new(OrderRepo::new());

    let handler = QueryMealItemHandler::new(order_repo.clone());

    let menu_item = MenuItem::new(String::from("fries"), String::from("345"));
    let meal_item_fries = MealItem::create(menu_item);
    let menu_item = MenuItem::new(String::from("burger"), String::from("789"));
    let meal_item_burger = MealItem::create(menu_item);
    let mut order = Order::new(1, vec![]);
    order.add_meal_items(vec![meal_item_fries.clone(), meal_item_burger.clone()]);
    order_repo.add(order);

    let response = handler.handle(1, meal_item_burger.id()).unwrap();

    let response = response.into_response();

    let status = response.status();
    let body = to_bytes(response.into_body()).await.unwrap();
    let body_bytes = body.to_vec();
    let actual_body: QueryMealItemResp = serde_json::from_slice(&*body_bytes).expect("failed to parse");
    let expected_body = QueryMealItemResp { data: MealItemResp::new(meal_item_burger) };

    assert_eq!(status, StatusCode::OK);
    assert_eq!(expected_body, actual_body);
}

#[tokio::test]
async fn test_query_meal_item_handler_handle_not_found() {
    let order_repo = Arc::new(OrderRepo::new());

    let handler = QueryMealItemHandler::new(order_repo.clone());

    let response = handler.handle(1, Uuid::new_v4()).unwrap();

    let response = response.into_response();

    let status = response.status();
    let body = to_bytes(response.into_body()).await.unwrap();
    let body_bytes = body.to_vec();
    let actual_body: ErrResp = serde_json::from_slice(&*body_bytes).expect("failed to parse");

    let expected_body: ErrResp = ErrResp { error_message: MESSAGE_ITEM_NOT_FOUND.to_string() };

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(expected_body, actual_body);
}
