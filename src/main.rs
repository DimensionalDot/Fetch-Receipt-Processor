use chrono::{Datelike, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    convert::Infallible,
    str::FromStr,
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

impl Reciept {
    fn points(&self) -> usize {
        let mut points = 0;

        points += self // 1 point per alphanumeric in retailer name
            .retailer
            .chars()
            .filter(|&c| c.is_alphanumeric())
            .count();

        let total: f32 = self
            .total
            .parse()
            .expect("total should have been checked to be a valid f32");

        if total % 1.0 == 0.0 {
            points += 50;
        }

        if total % 0.25 == 0.0 {
            points += 25;
        }

        points += self.items.len() / 2 * 5; // integer division

        for item in self.items.iter() {
            points += item.points();
        }

        if self.purchase_date.day() % 2 == 1 {
            points += 6
        }

        let min_time = NaiveTime::from_str("14:00").unwrap();
        let max_time = NaiveTime::from_str("16:00").unwrap();
        if min_time < self.purchase_time && self.purchase_time < max_time {
            points += 10;
        }

        return points;
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Item {
    short_description: String,
    price: String,
}

impl Item {
    pub fn points(&self) -> usize {
        if self.short_description.trim().chars().count() % 3 == 0 {
            let price: f32 = self
                .price
                .parse()
                .expect("price should have been checked to be a valid f32");

            (price * 0.2).ceil() as usize
        } else {
            0
        }
    }
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

    let points = warp::get()
        .and(warp::path!(usize / "points"))
        .and(with_reciepts(reciepts.clone()))
        .map(move |id: usize, reciepts: Arc<Mutex<Reciepts>>| {
            let reciepts = reciepts.lock().unwrap(); // TODO unwrap
            if let Some(reciept) = reciepts.get(&id) {
                let points = reciept.points();
                warp::reply::json(&json![{ "points": points}]).into_response()
            } else {
                StatusCode::NOT_FOUND.into_response()
            }
        });

    let routes = warp::path("reciepts").and(process.or(points));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_reciepts(
    reciepts: Arc<Mutex<Reciepts>>,
) -> impl Filter<Extract = (Arc<Mutex<Reciepts>>,), Error = Infallible> + Clone {
    warp::any().map(move || reciepts.clone())
}
