use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64
}

pub struct Uniques<T> {
    angles: Vec<T>
}

impl<T: std::cmp::PartialEq> Uniques<T> {
    pub fn new() -> Uniques<T> {
        Uniques {
            angles: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.angles.len()
    }

    pub fn add(&mut self, other: T) {
        if self.angles.contains(&other) {
            return;
        }

        self.angles.push(other);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Asteroid {
    pub location: Point
}

#[derive(Debug)]
pub struct Map {
    coordinates: Vec<Vec<Option<Asteroid>>>,
    asteroids: Vec<Asteroid>
}

impl Map {
    pub fn from_input(data: String) -> Map {

        let mut rows: Vec<Vec<Option<Asteroid>>> = Vec::new();
        let mut asteroids: Vec<Asteroid> = Vec::new();

        for (y, l) in data.split_whitespace().enumerate() {
            let l = l.trim();

            let mut row: Vec<Option<Asteroid>> = Vec::new();

            for (x, char) in l.chars().enumerate() {
                if char == '#' {
                    let ast = Asteroid {
                        location: Point{ x: x as i64, y: y as i64 }
                    };

                    row.push(Some(ast.clone()));
                    asteroids.push(ast);
                } else {
                    row.push(None);
                }
            }

            rows.push(row);
        }

        Map {
            coordinates: rows,
            asteroids
        }
    }

    pub fn distance(&self, p1: Point, p2: Point) -> f64 {
        (((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)) as f64).sqrt()
    }

    pub fn calculate_nth_shot(&mut self, base: Point, i: i64) -> Asteroid {
        let mut asteroids = self.asteroids.clone();

        asteroids.sort_by(|a, b| {
            let first_angle = self.atan2(base.clone(), a.location.clone());
            let second_angle = self.atan2(base.clone(), b.location.clone());

            first_angle
                .partial_cmp(&second_angle)
                .unwrap()
                .then_with(|| self.distance(a.location.clone(), base.clone())
                                .partial_cmp(
                                    &self.distance(b.location.clone(), base.clone())
                                ).unwrap())
        });

        let mut shot = 0;

        while !asteroids.is_empty() {
            let mut unreachable = Vec::new();
            let mut last_dir = None;

            for asteroid in &asteroids {
                let dir = self.atan2(base.clone(), asteroid.location.clone());

                if last_dir == Some(dir) {
                    unreachable.push(asteroid.clone());
                } else {
                    shot += 1;
                    if shot == i {
                        return asteroid.clone();
                    }

                    last_dir = Some(dir);
                }
            }

            asteroids = unreachable;
        }

        panic!("What.")
    }

    pub fn atan2(&self, point1: Point, point2: Point) -> f64 {
        -((point2.x - point1.x) as f64).atan2((point2.y - point1.y) as f64)
    }

    pub fn remove(&mut self, ast: Asteroid) {
        self.asteroids.remove_item(&ast);
        self.coordinates[ast.location.y as usize][ast.location.x as usize] = None;
    }

    pub fn calculate_line_of_sight(&mut self) -> HashMap<Asteroid, i64> {
        let mut ast_counts: HashMap<Asteroid, i64> = HashMap::new();

        for asteroid in &self.asteroids {
            let origin = asteroid.location.clone();

            let mut angles: Uniques<f64> = Uniques::new();

            for target in &self.asteroids {
                if asteroid == target {
                    continue;
                }

                let angle_rads = ((target.location.x - origin.x) as f64).atan2((origin.y - target.location.y) as f64);

                angles.add(angle_rads);
            }

            ast_counts.insert(asteroid.clone(), angles.len() as i64);
        }

        ast_counts
    }
}
