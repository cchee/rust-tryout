use std::collections::{HashMap, HashSet};

use bigdecimal::BigDecimal;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use utoipa::ToSchema;

use crate::db;
use crate::error_handler::CustomError;
use crate::schema::cost_items;
use crate::utils::check;

#[serde_as]
#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = cost_items)]
pub struct CostItem {
    pub name: String,
    #[serde_as(as = "DisplayFromStr")]
    pub price: BigDecimal,
    pub notes: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Queryable, Insertable, ToSchema)]
#[diesel(table_name = cost_items)]
pub struct CostItems {
    pub id: i64,
    pub name: String,
    #[serde_as(as = "DisplayFromStr")]
    pub price: BigDecimal,
    pub notes: String,
}

impl CostItems {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = db::connection()?;
        let cost_items = cost_items::table.load::<CostItems>(&mut conn)?;
        Ok(cost_items)
    }

    pub fn get(params: HashMap<String, String>) -> Result<Vec<Self>, CustomError> {
        let mut query = cost_items::table.into_boxed();

        if params.contains_key("id") {
            match check::validate_long(params.get("id").unwrap()) {
                Ok(id) => query = query.filter(cost_items::id.eq(id)),
                Err(error) => return Err(error),
            }
        }
        if params.contains_key("ids") {
            match check::parse_ids(params.get("ids").unwrap()) {
                Ok(ids) => {
                    let ids_clean: HashSet<i64> = ids.into_iter().collect();
                    query = query.filter(cost_items::id.eq_any(ids_clean));
                }
                Err(error) => return Err(error),
            }
        }
        if let Some(name) = params.get("name") {
            query = query.filter(cost_items::name.eq(name))
        }

        let mut conn = db::connection()?;
        let cost_items = match query.get_results(&mut conn) {
            Ok(cost_items) => cost_items,
            Err(e) => return Err(CustomError::from(e)),
        };

        Ok(cost_items)
    }

    pub fn find(id: i64) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let cost_item = cost_items::table.filter(cost_items::id.eq(id)).first(&mut conn)?;
        Ok(cost_item)
    }

    pub fn create(cost_item: CostItem) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let cost_item = CostItem::from(cost_item);
        let cost_item = diesel::insert_into(cost_items::table)
            .values(cost_item)
            .get_result(&mut conn)?;
        Ok(cost_item)
    }

    pub fn update(id: i64, cost_item: CostItem) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let cost_item = diesel::update(cost_items::table)
            .filter(cost_items::id.eq(id))
            .set(cost_item)
            .get_result(&mut conn)?;
        Ok(cost_item)
    }

    pub fn delete(id: i64) -> Result<usize, CustomError> {
        let mut conn = db::connection()?;
        let res = diesel::delete(cost_items::table.filter(cost_items::id.eq(id))).execute(&mut conn)?;
        Ok(res)
    }
}

impl CostItem {
    fn from(cost_item: CostItem) -> CostItem {
        CostItem {
            name: cost_item.name,
            price: cost_item.price,
            notes: cost_item.notes,
        }
    }
}