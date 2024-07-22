use serde::Serialize;

// can use enum
pub const MESSAGE_ORDER_NOT_FOUND: &str = "There are no orders associated with this table";
pub const MESSAGE_ITEM_NOT_FOUND: &str = "The specified meal items can't be found for this table";
pub const MESSAGE_ITEMS_REMOVAL_FAILED: &str = "Some items could not be removed as they are already started preparing, or completed.";
pub const MESSAGE_ORDER_REMOVAL_FAILED: &str = "Order cannot be removed as it is already started preparing, or completed";

#[derive(Serialize)]
pub struct ErrResp {
    pub message: String,
}