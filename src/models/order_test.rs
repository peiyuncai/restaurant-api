#[cfg(test)]
mod order_test {
    use crate::models::meal::{MealItem, MealItemStatus};
    use crate::models::menu::MenuItem;
    use crate::models::order::{Order, OrderStatus};

    #[test]
    fn test_add_meal_items() {
        let mut expected_meal_items = Vec::new();

        let menu_item = MenuItem::new(String::from("fries"), String::from("345"));
        let meal_item = MealItem::create(menu_item);
        let mut order = Order::new(1, vec![]);
        order.add_meal_items(vec![meal_item.clone()]);
        expected_meal_items.push(meal_item);

        let menu_item = MenuItem::new(String::from("burger"), String::from("789"));
        let meal_item = MealItem::create(menu_item);
        order.add_meal_items(vec![meal_item.clone()]);
        expected_meal_items.push(meal_item.clone());

        let mut actual_meal_items = order.get_meal_items();
        assert_eq!(expected_meal_items.len(), actual_meal_items.len());

        expected_meal_items.sort_by(|a, b| a.id().cmp(&b.id()));
        actual_meal_items.sort_by(|a, b| a.lock().unwrap().id().cmp(&b.lock().unwrap().id()));

        for (expected, actual) in expected_meal_items.iter().zip(actual_meal_items.iter()) {
            assert_eq!(expected.clone(), actual.lock().unwrap().clone());
        }
    }

    #[test]
    fn test_remove_meal_items() {
        let mut order = Order::new(1, vec![]);
        let menu_item_one = MenuItem::new(String::from("fries"), String::from("345"));
        let meal_item_one = MealItem::create(menu_item_one);
        let menu_item_two = MenuItem::new(String::from("burger"), String::from("789"));
        let mut meal_item_two = MealItem::create(menu_item_two);
        meal_item_two.update_state(MealItemStatus::Preparing);
        order.add_meal_items(vec![meal_item_one.clone(), meal_item_two.clone()]);
        let meal_item_ids = vec![meal_item_one.id(), meal_item_two.id()];

        for meal_item_arc in order.get_meal_items().iter() {
            assert!(!meal_item_arc.lock().unwrap().is_removed());
        }

        order.remove_meal_items(meal_item_ids);

        for meal_item_arc in order.get_meal_items().iter() {
            let meal_item = meal_item_arc.lock().unwrap();
            match meal_item.get_status() {
                MealItemStatus::Preparing | MealItemStatus::Completed => {
                    assert!(!meal_item.is_removed());
                }
                _ => {
                    assert!(meal_item.is_removed());
                }
            }
        }
    }

    #[test]
    fn test_get_meal_items() {
        let mut expected_meal_items = Vec::new();

        let menu_item = MenuItem::new(String::from("fries"), String::from("345"));
        let meal_item = MealItem::create(menu_item);
        let mut order = Order::new(1, vec![]);
        order.add_meal_items(vec![meal_item.clone()]);
        expected_meal_items.push(meal_item);

        let mut actual_meal_items = order.get_meal_items();
        assert_eq!(expected_meal_items.len(), actual_meal_items.len());

        for (expected, actual) in expected_meal_items.iter().zip(actual_meal_items.iter()) {
            assert_eq!(expected.clone(), actual.lock().unwrap().clone());
        }
    }

    #[test]
    fn test_get_meal_item() {
        let menu_item = MenuItem::new(String::from("fries"), String::from("345"));
        let meal_item = MealItem::create(menu_item);
        let mut order = Order::new(1, vec![]);
        order.add_meal_items(vec![meal_item.clone()]);

        if let Some(meal_item_arc) = order.get_meal_item(meal_item.id()) {
            assert_eq!(meal_item, meal_item_arc.lock().unwrap().clone());
        } else {
            panic!("meal item not found");
        }
    }

    #[test]
    fn test_get_order_status() {
        let menu_item = MenuItem::new(String::from("fries"), String::from("345"));
        let mut meal_item = MealItem::create(menu_item);
        let mut order = Order::new(1, vec![]);
        order.add_meal_items(vec![meal_item.clone()]);

        assert_eq!(OrderStatus::Received, order.get_order_status());

        meal_item.update_state(MealItemStatus::Preparing);
        order.add_meal_items(vec![meal_item.clone()]);
        assert_eq!(OrderStatus::Preparing, order.get_order_status());

        for meal_item_arc in order.get_meal_items() {
            meal_item_arc.lock().unwrap().update_state(MealItemStatus::Completed);
        }
        assert_eq!(OrderStatus::Completed, order.get_order_status());


        for meal_item_arc in order.get_meal_items() {
            meal_item_arc.lock().unwrap().remove();
        }
        assert_eq!(OrderStatus::Canceled, order.get_order_status());
    }
}