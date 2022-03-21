

#[deny(clippy::all)]
use serde::{Deserialize, Serialize};
use crate::Database;

#[derive(Deserialize, Serialize, Debug)]
pub struct Revocation {
    timestamp: Option<u64>,
    author: Option<u64>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Action {
    id: String,
    pub identifiers: Vec<String>,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    r#type: String,
   pub author: String,
    reason: String,
    timestamp: u64,
    revocation: Revocation,
    expiration: bool,
    pub playerName: String
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Notes {
    text: Option<String>,
    lastAdmin: Option<String>,
    tsLastEdit: Option<u64>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Player {
    pub license: String,
    pub name: String,
    tsLastConnection: u64,
    tsJoined: u64,
    playTime: f64,
    notes: Notes
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PendingWl {
    id: String,
    license: String,
    name: String,
    tsLastAttempt: u64
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TableLayout {
    pub players: Vec<Player>,
    pub actions: Vec<Action>,
    pub pendingWL: Vec<PendingWl>,
}

impl TableLayout {
    pub fn get_players(&mut self) -> Players {
        Players::new(&mut self.players)
    }
    pub fn get_actions(&mut self) -> Actions {
        Actions(&mut self.actions)
    }
    pub fn get_pendingwl(&self) -> &Vec<PendingWl> {
        &self.pendingWL
    }
}
#[derive(Debug)]
pub struct Players<'a>(pub &'a mut Vec<Player>);
#[derive(Debug)]
pub struct Actions<'a>(pub &'a mut Vec<Action>);

#[derive(Deserialize, Serialize, Debug)]
pub struct BanQuery {

}
#[derive(Deserialize, Serialize, Debug)]
pub struct GetActions {

}

#[derive(Deserialize, Serialize, Debug)]
pub struct HttpReqBody {
    pub action: String, //either ADD_BAN or GET_ACTIONS
    pub get: Option<Vec<String>>,
    pub banQuery: Option<BanQuery>
}