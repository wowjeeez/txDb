use crate::structs::{Player, Players};

impl Players<'_> {
    pub fn new(vec: &mut Vec<Player>) -> Players {
        Players(vec)
    }
    pub fn get_by_name(&self, name: String) -> Option<&Player> {
        let pos = self.0.iter().position(|e| e.name == name);
        return if pos.is_some() {
            Some(self.0.get(pos.unwrap()).unwrap())
        } else {
            None
        }
    }
    pub fn size(&self) -> usize {
        std::mem::size_of_val(&self.0)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}