use super::r#type::{Flag, Type};

#[derive(Debug)]
pub struct List(Vec<Type>);

impl List {
    pub fn add(&mut self, from: usize, to: usize, flag: Flag) {
        let move_type = Type::new(from, to, flag);

        self.0.push(move_type);
    }
}
