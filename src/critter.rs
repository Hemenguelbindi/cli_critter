pub struct Critter {
    pub name: String,
    pub health: i32,
}

impl Critter {
    pub fn new(name: &str, health: i32) -> Critter {
        Critter {
            name: name.to_string(),
            health,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}