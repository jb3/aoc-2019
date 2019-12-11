use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Location {
    pub x: i64,
    pub y: i64
}

impl From<(i64, i64)> for Location {
    fn from(data: (i64, i64)) -> Location {
        Location {
            x: data.0,
            y: data.1
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum Colour {
    Black,
    White
}

#[derive(Debug)]
pub struct Canvas {
    pub painted: HashMap<Location, Colour>,
    pub history: HashSet<Location>
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas {
            painted: HashMap::new(),
            history: HashSet::new()
        }
    }

    pub fn get_colour(&self, location: &Location) -> Colour {
        *self.painted.get(location).unwrap_or(&Colour::Black)
    }

    pub fn set_colour(&mut self, location: &Location, colour: Colour) {
        self.history.insert(*location);
        self.painted.insert(location.clone(), colour);
    }
}
