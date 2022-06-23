use super::r#type::Type;

pub struct List(Vec<Type>);

impl List {
    pub fn add(&mut self, from: u16, to: u16, flag: u16) {
        let move_type = Type::new(from, to, flag);

        self.0.push(move_type);
    }
}
