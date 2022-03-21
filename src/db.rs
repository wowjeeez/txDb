use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use crate::{get_db, Player};
use crate::structs::{Action, PendingWl, TableLayout};
#[derive(Debug)]
pub struct Database {
    path: PathBuf,
    data: Option<TableLayout>,

}

impl Database {
    pub fn new(path: &str) -> Database {
        let pb = PathBuf::from(path);
        if !pb.exists() {
            std::fs::write(&pb, r#"{"players": [], "actions": [], "pendingWL": []}"#);
        }

        let mut db = Database {path: pb, data: None};
        let iter = db.open_f();
        let rd = serde_json::from_reader(iter);
        let tables: TableLayout = rd.unwrap();
        db.data = Some(tables);
        db

    }
    fn open_f(&self) -> BufReader<File> {
        let f = BufReader::new(OpenOptions::new().write(true).read(true).open(&self.path).unwrap());
        f
    }
    pub(crate) fn get_tbls<'t>(&'t mut self) -> &'t mut TableLayout {
            self.data.as_mut().unwrap()
    }

    pub fn write_to_disk(&mut self) {
        println!("Writing new struct: {:?} to disk", &self.data);
        let str = serde_json::to_string(&self.data.as_ref().unwrap()).unwrap();
        std::fs::write(&self.path, str).unwrap();
    }
    pub fn push_action(&mut self, action: Action) {
        self.data.as_mut().unwrap().actions.push(action);
        self.write_to_disk();
    }

    pub fn push_player(&mut self, player: Player) {
        self.data.as_mut().unwrap().players.push(player);
        self.write_to_disk();
    }

    pub fn push_wl(&mut self, wl: PendingWl) {
        self.data.as_mut().unwrap().pendingWL.push(wl);
        self.write_to_disk();
    }


}
