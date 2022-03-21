mod structs;
mod db;
mod player;
mod action;



use warp::Filter;
use crate::db::{Database};

use std::sync::{Arc, Mutex};
use std::time::Instant;
use crate::structs::HttpReqBody;

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let db = Database::new("mock.json");
    println!("Database parsed in {} ms", now.elapsed().as_millis());
    let state = Arc::new(Mutex::new(db));
    let routes = warp::path("fetch").and(warp::body::bytes()).map(move |bytes: bytes::Bytes| {
        let now = Instant::now();
        let bvec = bytes.to_vec();
        let str = std::str::from_utf8(&bvec).unwrap_or("");
        let parsed: HttpReqBody = serde_json::from_str(str).unwrap();
        return if parsed.action == "ADD_BAN".to_string() {
            "UNIMPLEMENTED".to_string()
        } else if parsed.action == "GET_ACTIONS" {
            let pload = parsed.get.unwrap_or(vec![]);
            let mut lock = state.lock().unwrap();
            let actions = lock.get_tbls().get_actions();
            let res = actions.get_by_idents(pload);
            format!(r#""status": "ok", "took": {}, "resp": {}"#, now.elapsed().as_nanos(), serde_json::to_string(&res).unwrap())
        } else {
            format!(r#""status": "failed", "reason": "INVALID_ACTION""#)
        }
    });
    warp::serve(routes).run(([127, 0, 0, 1], 40121)).await;
}
