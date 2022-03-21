mod structs;
mod db;
mod player;
mod action;



use warp::Filter;
use crate::db::{Database};

use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Instant;
use lazy_static::lazy_static;
use crate::structs::{HttpReqBody, Player};


#[tokio::main]
async fn main() {
    let now = Instant::now();
    println!("Database parsed in {} ms", now.elapsed().as_millis());
    /*
    let mut db = state.lock().unwrap();
    db.push_player(serde_json::from_str(r#"  {
    "license": "9b9fc300cc65d22ad3b536175a4d15c0e4933753",
    "name": "Tabarra",
    "playTime": 4591,
    "tsJoined": 1590812869,
    "tsLastConnection": 1632671882,
    "notes": {
      "text": "",
      "lastAdmin": null,
      "tsLastEdit": null
    }
  }"#).unwrap());
    drop(db);
     */
    let state = Arc::new(Mutex::new(Database::new("test.json")));
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