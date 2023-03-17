use std::collections::HashMap;

use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

use crate::cost_items::{CostItem, CostItems};
use crate::error_handler::CustomError;
use crate::utils::check;
use crate::utils::response;

#[utoipa::path(
    get,
    path = "/cost_items",
    responses(
        (status = 200, description = "Get all cost items", body = inline(response::CostItemsResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[get("/cost_items")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let cost_items = web::block(CostItems::find_all).await.unwrap();
    Ok(HttpResponse::Ok().json(cost_items))
}

#[utoipa::path(
    get,
    path = "/cost_items/filter",
    responses(
        (status = 200, description = "Get cost items filtered with url params", body = inline(response::CostItemsResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    ),
    params(
        ("id" = Option<i64>, Query, description = "CostItem database id"),
        ("ids" = Option<String>, Query, description = "CostItems database comma separated ids example (1,2,3)"),
        ("name" = Option<String>, Query,  description = "CostItem Name"),
        ("price" = Option<BigDecimal>, Query,  description = "CostItem Price"),
        ("notes" = Option<String>, Query, description = "CostItem Notes"),
    )
)]
#[get("/cost_items/filter")]
async fn filter(_param: web::Query<HashMap<String, String>>) -> Result<HttpResponse, CustomError> {
    let params = _param.into_inner();

    let cost_items = if params.is_empty() {
        web::block(CostItems::find_all).await.unwrap()
    } else {
        match check::validate_cost_item_params(&params) {
            Ok(..) => web::block(move || CostItems::get(params)).await.unwrap(),
            Err(err) => return Err(err),
        }
    };

    Ok(HttpResponse::Ok().json(cost_items))
}

#[utoipa::path(
    get,
    path = "/cost_items/{id}",
    responses(
        (status = 200, description = "Get a cost_item identifies with id", body = inline(CostItems)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[get("/cost_items/{id}")]
async fn find(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let cost_item = CostItems::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(cost_item))
}

#[utoipa::path(
    post,
    path = "/cost_items",
    responses(
        (status = 200, description = "Create a new cost_item", body = inline(response::CostItemResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[post("/cost_items")]
async fn create(cost_item: web::Json<CostItem>) -> Result<HttpResponse, CustomError> {
    let cost_item = CostItems::create(cost_item.into_inner())?;
    Ok(HttpResponse::Ok().json(cost_item))
}

#[utoipa::path(
    put,
    path = "/cost_items/{id}",
    responses(
    (status = 200, description = "Modify a new cost_item", body = inline(response::CostItemResponse)),
    (status = 400, description = "Error", body = inline(response::ErrorResponse)),
    (status = 404, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[put("/cost_items/{id}")]
async fn update(id: web::Path<i64>, cost_item: web::Json<CostItem>) -> Result<HttpResponse, CustomError> {
    let cost_item = CostItems::update(id.into_inner(), cost_item.into_inner())?;
    Ok(HttpResponse::Ok().json(cost_item))
}

#[utoipa::path(
    delete,
    path = "/cost_items/{id}",
    responses(
        (status = 200, description = "Delete a new cost_item", body = inline(response::DeleteResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[delete("/cost_items/{id}")]
async fn delete(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let deleted_cost_item = CostItems::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_cost_item })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(filter);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
    config.service(delete);
}