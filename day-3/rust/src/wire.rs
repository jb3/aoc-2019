use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64
}

#[derive(Clone, Debug)]
pub struct Wire {
    points: Vec<Point>
}

impl Wire {
    pub fn new(points: Vec<Point>) -> Wire {
        Wire {
            points
        }
    }

    pub fn to_set(self) -> HashSet<Point> {
        self.points.into_iter().collect()
    }

    pub fn find_steps_to(self, point: &Point) -> i64 {
        self.points.iter().position(|p| p == point).unwrap().try_into().unwrap()
    }
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point {
            x, y
        }
    }

    pub fn up(&mut self, distance: i64) {
        self.y += distance;
    }

    pub fn down(&mut self, distance: i64) {
        self.y -= distance;
    }

    pub fn right(&mut self, distance: i64) {
        self.x += distance;
    }

    pub fn left(&mut self, distance: i64) {
        self.x -= distance;
    }

    pub fn manhattan_distance(&self, other_point: &Point) -> i64 {
        (self.x - other_point.x).abs() + (self.y - other_point.y).abs()
    }
}
