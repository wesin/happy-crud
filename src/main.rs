use crate::config::get_config;
use server::db_center::init_db;
use std::{collections::HashMap, net::ToSocketAddrs};
use utils::ws_error::handle_rejection;

use crate::server::person::*;
use warp::Filter;
mod config;
mod entity;
mod model;
mod server;
mod utils;

fn init_env() {
    log4rs::init_file("log4rs.yml", Default::default()).expect("lost file log4rs.yml");
}

#[tokio::main]
async fn main() {
    init_env();
    init_db().await;

    // 跨域
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST"]);

    let add_person = warp::post()
        .and(warp::path("add_person"))
        .and(warp::body::json())
        .and_then(add_persion);
    let query_persons = warp::post()
        .and(warp::path("query_persons"))
        .and(warp::body::json())
        .and_then(query_persons);
    let get_person_info = warp::get()
        .and(warp::path("get_person_info"))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(get_person_info);
    warp::serve(
        add_person
            .or(query_persons)
            .or(get_person_info)
            .recover(handle_rejection)
            .with(cors),
    )
    .run(
        get_config()
            .http
            .address
            .clone()
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap(),
    )
    .await;
}
