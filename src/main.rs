use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    ops::DerefMut,
    sync::{Arc, Mutex},
};
use warp::Filter;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Reciept {
    retailer: String,
    purchase_date: NaiveDate,
    purchase_time: NaiveTime,
    items: Vec<Item>,
    total: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Item {
    short_description: String,
    price: String,
}

#[tokio::main]
async fn main() {
    let reciepts: HashMap<usize, Reciept> = HashMap::new();
    let reciepts = Arc::new(Mutex::new(reciepts));

    let process = warp::post()
        .and(warp::path("process"))
        .and(warp::path::end())
        .and(warp::body::json())
        .map(move |reciept: Reciept| {
            let reciepts = Arc::clone(&reciepts);
            let mut reciepts = reciepts
                .lock()
                .expect("reciepts shouldn't be locked on this thread");
            let reciepts = reciepts.deref_mut();
            let id = reciepts.len();

            reciepts.insert(id, reciept);

            warp::reply::json(&json![{ "id": id }])
        });

    let routes = process;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
