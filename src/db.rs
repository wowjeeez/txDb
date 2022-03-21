use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use std::path::PathBuf;
use crate::structs::TableLayout;

pub struct Database {
    path: PathBuf,
    data: Option<TableLayout>
}

pub enum Tables {
    PLAYERS,
    ACTIONS,
    PWLS
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
}
