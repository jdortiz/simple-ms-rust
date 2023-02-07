use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use tracing::debug;
use uuid::Uuid;

use crate::order_store::OrderStore;

use super::{request::AddItem, response::Order};

type DataState = Arc<dyn OrderStore + Sync + Send>;

const USER_ID: Uuid = Uuid::from_u128(0x5afb91d8_555d_45d7_a517_ece1b6655b42);

//#[axum::debug_handler]
pub async fn create(State(state): State<DataState>) -> (StatusCode, Json<Option<Order>>) {
    debug!("Creating order");
    if let Ok(order) = state.create_order(USER_ID).await {
        (StatusCode::CREATED, Json(Some(Order::from(order))))
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
    }
}

pub async fn list() -> (StatusCode, Json<Option<Vec<Order>>>) {
    debug!("Listing orders");
    (StatusCode::FORBIDDEN, Json(None))
}

pub async fn get(Path(id): Path<Uuid>) -> (StatusCode, Json<Option<Order>>) {
    debug!("Get order id: {id}");
    (StatusCode::FORBIDDEN, Json(None))
}

pub async fn add_item(Path(id): Path<Uuid>, Json(request): Json<AddItem>) -> StatusCode {
    debug!(
        "Add item to order id: {}: product_id={} quantity={}",
        id, request.product_id, request.quantity
    );
    StatusCode::FORBIDDEN
}

pub async fn delete_item(Path((id, index)): Path<(Uuid, usize)>) -> StatusCode {
    debug!("Delete item {index} from order id: {id}");
    StatusCode::FORBIDDEN
}
