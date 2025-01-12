use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    convert::Infallible,
    sync::{Arc, Mutex},
};
use warp::{http::StatusCode, reply::Reply, Filter};

type Reciepts = HashMap<usize, Reciept>;

#[derive(Deserialize)]
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
    let reciepts: Reciepts = HashMap::new();
    let reciepts = Arc::new(Mutex::new(reciepts));
    let process = warp::post()
        .and(warp::path("process"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_reciepts(reciepts.clone()))
        .map(move |reciept: Reciept, reciepts: Arc<Mutex<Reciepts>>| {
            if let Ok(mut reciepts) = reciepts.lock() {
                let id = reciepts.len();
                reciepts.insert(id, reciept); // currently overwriting on collision

                warp::reply::json(&json![{ "id": id }]).into_response()
            } else {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }

        });

    let routes = process;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_reciepts(
    reciepts: Arc<Mutex<Reciepts>>,
) -> impl Filter<Extract = (Arc<Mutex<Reciepts>>,), Error = Infallible> + Clone {
    warp::any().map(move || reciepts.clone())
}
