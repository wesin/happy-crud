use std::{collections::HashMap, str::FromStr};

use super::db_center::get_db;
use crate::entity::prelude::Person;
use crate::model::page::{Page, PageQueryDO};
use crate::model::person::PersonQueryDO;
use crate::{
    entity::person,
    model::person::PersonAddDO,
    utils::{ws_error::WSError, ws_response::WSResponse},
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};
use uuid::Uuid;
use warp::{Rejection, Reply};

pub async fn add_persion(item: PersonAddDO) -> Result<impl Reply, Rejection> {
    let person_add = person::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(item.name.clone()),
        age: Set(item.age),
        desc: Set(item.desc.clone()),
    };
    let r = person_add
        .insert(get_db())
        .await
        .map_err(WSError::SqlError)?;
    Ok(WSResponse::data(r))
}

pub async fn get_person_info(args: HashMap<String, String>) -> Result<impl Reply, Rejection> {
    let person_id = args.get("person_id");
    if person_id.is_none() {
        return Ok(WSError::RequestParamError("person_id".to_string()).into_response());
    }
    let p = Person::find_by_id(Uuid::parse_str(person_id.unwrap()).unwrap())
        .one(get_db())
        .await
        .map_err(WSError::SqlError)?;
    Ok(WSResponse::data(p).into_response())
}

pub async fn query_persons(
    request: PageQueryDO<PersonQueryDO>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut query = Person::find();
    if let Some(item) = &request.item {
        if let Some(name) = &item.name {
            query = query.filter(person::Column::Name.contains(name))
        }
    }
    let page = query.paginate(get_db(), request.page_size);
    let items = page
        .fetch_page(request.page_index)
        .await
        .map_err(WSError::SqlError)?;
    let mut has_next = false;
    if let Ok(next) = page.fetch_page(request.page_index + 1).await {
        if !next.is_empty() {
            has_next = true;
        }
    }
    let mut result = Page::<person::Model>::new(items, has_next);
    if request.get_all_count {
        if let Ok(count) = page.num_items().await {
            result.set_count(count);
        }
    }
    // success(Some(result))
    Ok(WSResponse::data(result).into_response())
}
