use serde::{Deserialize, Serialize};

// can use enum
pub const MESSAGE_ORDER_NOT_FOUND: &str = "There are no order associated with this table";
pub const MESSAGE_ITEM_NOT_FOUND: &str = "The specified meal item can't be found for this table";
pub const MESSAGE_ITEMS_PARTIALLY_REMOVED: &str = "If items can be removed, they are removed; otherwise, no operation since meals are either started preparing, completed, or simply not existed.";
pub const MESSAGE_ORDER_REMOVAL_CONFLICT: &str = "Order cannot be removed as it is already started preparing, or completed";
pub const MESSAGE_ORDER_ADD_CONFLICT: &str = "Order cannot be created since there is an ongoing order for this table";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ErrResp {
    pub error_message: String,
}