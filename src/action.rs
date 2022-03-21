use std::sync::MutexGuard;
use crate::{Database, get_db};
use crate::structs::{Action, Actions, TableLayout};

impl Actions<'_> {
    pub fn new(vec: &mut Vec<Action>) -> Actions {
        Actions(vec)
    }
    pub fn size(&self) -> usize {
        std::mem::size_of_val(&self.0)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn get_by_idents(&self, idents: Vec<String>) -> Vec<&Action> {
            let mut viter = idents.iter();
            let found = self.0.iter().filter(|val| viter.position(|id| val.identifiers.contains(id)).is_some()).collect::<Vec<&Action>>();
            found
    }
    //almost impossible to get to work because mutex locks, use `db.push_action` instead
    pub fn add(&mut self, action: Action) {
            get_db().push_action(action);
    }
}