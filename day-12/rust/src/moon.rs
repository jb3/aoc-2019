use std::ops::{Add, AddAssign};

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub struct Vector3 {
    pub x: i64,
    pub y: i64,
    pub z: i64
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Moon {
    pub location: Vector3,
    pub velocity: Vector3
}

impl Moon {
    pub fn parse(text: String) -> Moon {
        let s = text.trim_matches(|a| a == '<' || a == '>');

        let comps = s.split_whitespace().map(|x| x.trim().to_string()).collect::<Vec<String>>();

        let nums = comps.iter().map(|x| x.trim_end_matches(',').to_string()).collect::<Vec<String>>();

        let nums_parsed = nums.iter().map(|x| &x[2..]).map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        if let &[x, y, z] = &nums_parsed[..] {
            Moon {
                location: Vector3 {
                    x,
                    y,
                    z
                },
                velocity: Vector3 {
                    x: 0,
                    y: 0,
                    z: 0
                }
            }
        } else {
            panic!("Expected x,y,z");
        }
    }

    pub fn apply_velocity(&mut self) {
        self.location = self.location + self.velocity;
    }
}
