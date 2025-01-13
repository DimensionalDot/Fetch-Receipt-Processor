use chrono::{Datelike, NaiveDate, NaiveTime};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    convert::Infallible,
    str::FromStr,
    sync::{Arc, Mutex},
};
use uuid::Uuid;
use warp::{
    http::StatusCode,
    reply::{self, Reply},
    Filter,
};

type Reciepts = HashMap<Uuid, Reciept>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Reciept {
    retailer: String,
    purchase_date: NaiveDate,
    purchase_time: NaiveTime,
    items: Vec<Item>,
    #[serde(deserialize_with = "deserialize_cost")]
    total: f32,
}

impl Reciept {
    fn points(&self) -> usize {
        let mut points = 0;

        points += self // 1 point per alphanumeric in retailer name
            .retailer
            .chars()
            .filter(|&c| c.is_alphanumeric())
            .count();

        if self.total % 1.0 == 0.0 {
            points += 50;
        }

        if self.total % 0.25 == 0.0 {
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
    #[serde(deserialize_with = "deserialize_cost")]
    price: f32,
}

impl Item {
    pub fn points(&self) -> usize {
        if self.short_description.trim().chars().count() % 3 == 0 {
            (self.price * 0.2).ceil() as usize
        } else {
            0
        }
    }
}

#[tokio::main]
async fn main() {
    let reciepts: Reciepts = HashMap::new();
    let reciepts = Arc::new(Mutex::new(reciepts));
    let process = warp::path!("process")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_reciepts(reciepts.clone()))
        .map(add_reciept);

    let points = warp::path!(Uuid / "points")
        .and(warp::get())
        .and(with_reciepts(reciepts.clone()))
        .map(get_points);

    let routes = warp::path("reciepts").and(process.or(points));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_reciepts(
    reciepts: Arc<Mutex<Reciepts>>,
) -> impl Filter<Extract = (Arc<Mutex<Reciepts>>,), Error = Infallible> + Clone {
    warp::any().map(move || reciepts.clone())
}

fn add_reciept(reciept: Reciept, reciepts: Arc<Mutex<Reciepts>>) -> reply::Response {
    if let Ok(mut reciepts) = reciepts.lock() {
        let id = Uuid::new_v4();
        reciepts.insert(id, reciept);

        warp::reply::json(&json![{ "id": id.to_string() }]).into_response()
    } else {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

fn get_points(id: Uuid, reciepts: Arc<Mutex<Reciepts>>) -> reply::Response {
    if let Ok(reciepts) = reciepts.lock() {
        if let Some(reciept) = reciepts.get(&id) {
            let points = reciept.points();
            warp::reply::json(&json![{ "points": points}]).into_response()
        } else {
            StatusCode::NOT_FOUND.into_response()
        }
    } else {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

fn deserialize_cost<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    struct CostVisitor;

    impl<'de> Visitor<'de> for CostVisitor {
        type Value = f32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string containing a floating point number")
        }

        fn visit_str<E>(self, cost: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            cost.parse().map_err(E::custom)
        }
    }

    deserializer.deserialize_str(CostVisitor)
}
