use serde::Serialize;

pub const MESSAGE_ORDER_NOT_FOUND: &str = "There are no orders associated with this table";
pub const MESSAGE_ITEM_NOT_FOUND: &str = "The specified meal items can't be found for this table";

#[derive(Serialize)]
pub struct ErrResp {
    pub message: String,
}