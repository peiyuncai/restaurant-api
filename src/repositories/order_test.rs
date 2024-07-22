#[cfg(test)]
mod order_test {
    use crate::models::meal::{MealItem, MealItemStatus};
    use crate::models::menu::MenuItem;
    use crate::models::order::Order;
    use crate::repositories::order::OrderRepo;

    #[test]
    fn test_add() {
        let repo = OrderRepo::new();

        let menu_item = MenuItem::new(String::from("fries"), String::from("34.5"));
        let order = Order::new(1, vec![menu_item]);

        repo.add(order.clone());

        if let Some(order_arc) = repo.get_order_by_table_id(1) {
            let fetched_order = order_arc.lock().unwrap();
            let fetched_meal_item = fetched_order.get_meal_items().get(0).unwrap().lock().unwrap().clone();
            let meal_item = order.get_meal_items().get(0).unwrap().lock().unwrap().clone();
            assert_eq!(order.get_table_id(), fetched_order.get_table_id());
            assert_eq!(order.get_total_price(), fetched_order.get_total_price());
            assert_eq!(meal_item, fetched_meal_item);
        } else {
            panic!("Order not found");
        }
    }

    #[test]
    fn test_get_order_by_table_id() {
        let repo = OrderRepo::new();

        assert!(repo.get_order_by_table_id(1).is_none());
        assert!(repo.get_order_by_table_id(2).is_none());

        repo.add(Order::new(1, vec![]));

        assert!(repo.get_order_by_table_id(1).is_some());
        assert!(repo.get_order_by_table_id(2).is_none());
    }

    #[test]
    fn test_get_order_meal_item() {
        let repo = OrderRepo::new();

        let menu_item = MenuItem::new(String::from("fries"), String::from("345"));
        let meal_item = MealItem::create(menu_item);
        let mut order = Order::new(1, vec![]);
        order.add_meal_items(vec![meal_item.clone()]);

        repo.add(order.clone());

        if let Some(meal_item_arc) = repo.get_order_meal_item(1, meal_item.id()) {
            let fetched_meal_item = meal_item_arc.lock().unwrap().clone();
            assert_eq!(meal_item, fetched_meal_item);
        } else {
            panic!("meal item not found")
        }
    }

    #[test]
    fn test_update_order_meal_item_status() {
        let repo = OrderRepo::new();

        let menu_item = MenuItem::new(String::from("fries"), String::from("345"));
        let meal_item = MealItem::create(menu_item);
        let mut order = Order::new(1, vec![]);
        order.add_meal_items(vec![meal_item.clone()]);

        repo.add(order);

        if let Some(meal_item_arc) = repo.get_order_meal_item(1, meal_item.id()) {
            let meal_item_arc = meal_item_arc.lock().unwrap().clone();
            assert_eq!(MealItemStatus::Received, meal_item_arc.get_status());
        } else {
            panic!("meal item not found")
        }

        let existed = repo.update_order_meal_item_status(1, meal_item.id(), MealItemStatus::Preparing);
        assert!(existed);

        if let Some(meal_item_arc) = repo.get_order_meal_item(1, meal_item.id()) {
            let meal_item_arc = meal_item_arc.lock().unwrap().clone();
            assert_eq!(MealItemStatus::Preparing, meal_item_arc.get_status());
        } else {
            panic!("meal item not found")
        }
    }

    #[test]
    fn test_add_order_meal_items() {
        let repo = OrderRepo::new();
        let mut expected_meal_items = Vec::new();

        let existed = repo.add_order_meal_items(1, vec![]);
        assert!(!existed);

        let menu_item = MenuItem::new(String::from("fries"), String::from("345"));
        let meal_item = MealItem::create(menu_item);
        let mut order = Order::new(1, vec![]);
        order.add_meal_items(vec![meal_item.clone()]);
        expected_meal_items.push(meal_item);

        repo.add(order);

        let menu_item = MenuItem::new(String::from("burger"), String::from("789"));
        let meal_item = MealItem::create(menu_item);
        expected_meal_items.push(meal_item.clone());

        let existed = repo.add_order_meal_items(1, vec![meal_item.clone()]);
        assert!(existed);

        if let Some(order_arc) = repo.get_order_by_table_id(1) {
            let order = order_arc.lock().unwrap().clone();
            let mut actual_meal_items = order.get_meal_items();
            assert_eq!(expected_meal_items.len(), actual_meal_items.len());

            expected_meal_items.sort_by(|a, b| a.id().cmp(&b.id()));
            actual_meal_items.sort_by(|a, b| a.lock().unwrap().id().cmp(&b.lock().unwrap().id()));

            for (expected, actual) in expected_meal_items.iter().zip(actual_meal_items.iter()) {
                assert_eq!(expected.clone(), actual.lock().unwrap().clone());
            }
        } else {
            panic!("order  not found")
        }
    }

    #[test]
    fn test_remove_order_meal_items() {
        let repo = OrderRepo::new();

        let mut order = Order::new(1, vec![]);
        let menu_item_one = MenuItem::new(String::from("fries"), String::from("345"));
        let meal_item_one = MealItem::create(menu_item_one);
        let menu_item_two = MenuItem::new(String::from("burger"), String::from("789"));
        let meal_item_two = MealItem::create(menu_item_two);
        order.add_meal_items(vec![meal_item_one.clone(), meal_item_two.clone()]);
        let meal_item_ids = vec![meal_item_one.id(), meal_item_two.id()];

        repo.add(order);

        if let Some(order_arc) = repo.get_order_by_table_id(1) {
            let order = order_arc.lock().unwrap().clone();
            for meal_item_arc in order.get_meal_items().iter() {
                assert!(!meal_item_arc.lock().unwrap().is_removed());
            }
        } else {
            panic!("order  not found")
        }

        repo.remove_order_meal_items(1, meal_item_ids);

        if let Some(order_arc) = repo.get_order_by_table_id(1) {
            let order = order_arc.lock().unwrap().clone();
            for meal_item_arc in order.get_meal_items().iter() {
                assert!(meal_item_arc.lock().unwrap().is_removed());
            }
        } else {
            panic!("order  not found")
        }
    }

    #[test]
    fn test_remove_order() {
        let repo = OrderRepo::new();

        // Given Order in Received status, when attempting cancellation, order should be canceled.
        let mut order = Order::new(1, vec![]);
        let menu_item = MenuItem::new(String::from("fries"), String::from("345"));
        let meal_item = MealItem::create(menu_item);
        order.add_meal_items(vec![meal_item.clone()]);

        repo.add(order);

        let (removed, existed) = repo.remove_order(1);
        assert!(removed);
        assert!(existed);

        // Given Order in Preparing status, when attempting cancellation, order should not be canceled.
        let mut order = Order::new(1, vec![]);
        let menu_item = MenuItem::new(String::from("fries"), String::from("345"));
        let mut meal_item = MealItem::create(menu_item);
        meal_item.update_state(MealItemStatus::Preparing);
        order.add_meal_items(vec![meal_item.clone()]);

        repo.add(order);

        let (removed, existed) = repo.remove_order(1);
        assert!(!removed);
        assert!(existed);
    }
}
