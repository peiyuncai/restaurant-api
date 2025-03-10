use std::sync::Arc;
use warp::reply::Reply;
use warp::hyper::body::to_bytes;
use uuid::Uuid;
use warp::http::StatusCode;
use crate::usecases::handlers::add_order::{AddOrderHandler, AddOrderReq, AddOrderResp, MenuItemReq};
use crate::libraries::mocks::thread_pool_mock::MockThreadPool;
use crate::repositories::order::OrderRepo;

#[tokio::test]
async fn test_add_order_handler_handle_success() {
    let order_repo = Arc::new(OrderRepo::new());
    let thread_pool = Arc::new(MockThreadPool::new());

    let handler = AddOrderHandler::new(order_repo.clone(), thread_pool.clone());

    let req = AddOrderReq {
        table_id: 1,
        menu_items: vec![
            MenuItemReq {
                menu_item_id: Uuid::new_v4(),
                name: String::from("fries"),
                price: String::from("345"),
            },
            MenuItemReq {
                menu_item_id: Uuid::new_v4(),
                name: String::from("burger"),
                price: String::from("789"),
            },
        ],
    };

    let response = handler.handle(req).unwrap();

    let response = response.into_response();

    let status = response.status();
    let body = to_bytes(response.into_body()).await.unwrap();
    let body_bytes = body.to_vec();
    let actual_body: AddOrderResp = serde_json::from_slice(&*body_bytes).expect("failed to parse");

    thread_pool.wait();
    assert_eq!(status, StatusCode::OK);
    assert_eq!(2, thread_pool.get_count());
    // can be improved here
    assert_eq!("1134", actual_body.data.total_price);
    assert_eq!("Received", actual_body.data.status);
    assert_eq!(2, actual_body.data.meal_items.len());
}
