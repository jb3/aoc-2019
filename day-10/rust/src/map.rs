use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64
}

impl Point {
    pub fn down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1
        }
    }

    pub fn up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1
        }
    }

    pub fn left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y
        }
    }

    pub fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y
        }
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

    fn get_directional_intersections(&self, point: &Point) -> (Vec<Asteroid>, Vec<Asteroid>) {
        let mut intersections: Vec<Asteroid> = Vec::new();
        let mut blocked: Vec<Asteroid> = Vec::new();
        // Up
        let mut p = point.clone();
        let mut found = false;

        for _ in 0..p.y {
            p = p.up();

            if let Some(ast) = &self.coordinates[p.y as usize][p.x as usize] {
                if found == false {
                    intersections.push(ast.clone());
                    found = true;
                }
                blocked.push(ast.clone())
            }
        }

        // Down
        let mut p = point.clone();
        let mut found = false;

        for _ in 0..(self.coordinates.len() - p.y as usize - 1) {
            p = p.down();

            if let Some(ast) = &self.coordinates[p.y as usize][p.x as usize] {
                if found == false {
                    intersections.push(ast.clone());
                    found = true;
                }
                blocked.push(ast.clone())
            }
        }

        // Left

        let mut p = point.clone();
        let mut found = false;

        for _ in 0..p.x {
            p = p.left();

            if let Some(ast) = &self.coordinates[p.y as usize][p.x as usize] {
                if found == false {
                    intersections.push(ast.clone());
                    found = true;
                }
                blocked.push(ast.clone())
            }
        }

        // Right

        let mut p = point.clone();
        let mut found = false;

        for _ in 0..(self.coordinates[0].len() - (p.x as usize) - 1) {
            p = p.right();

            if let Some(ast) = &self.coordinates[p.y as usize][p.x as usize] {
                if found == false {
                    intersections.push(ast.clone());
                    found = true;
                }
                blocked.push(ast.clone())
            }
        }

        (blocked, intersections)
    }

    fn get_points_between(&self, p1: Point, p2: Point) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        let m: f64 = ((p1.y - p2.y) as f64) / ((p1.x - p2.x) as f64);
        let c = (p1.y as f64) - (m * p1.x as f64);

        for x in p1.x + 1..p2.x {
            let xf = x as f64;

            let y = (m*xf) + c;

            if y.trunc() == y {
                points.push(Point {
                    x: x,
                    y: y as i64
                })
            }
        }

        points
    }

    pub fn calculate_line_of_sight(&mut self) -> HashMap<Asteroid, i64> {
        let mut ast_counts: HashMap<Asteroid, i64> = HashMap::new();

        for asteroid in &self.asteroids {
            let origin = asteroid.location.clone();

            let (blocked, mut intersections): (Vec<Asteroid>, Vec<Asteroid>) = self.get_directional_intersections(&origin);

            'target: for asteroid_target in &self.asteroids {
                if asteroid == asteroid_target {
                    continue;
                }

                if blocked.contains(asteroid_target) {
                    continue;
                }

                let target = asteroid_target.location.clone();

                let other_points: Vec<Point>;

                if origin.x < target.x {
                    other_points = self.get_points_between(origin.clone(), target);
                } else {
                    other_points = self.get_points_between(target, origin.clone());
                }

                for point in other_points {
                    if let Some(_) = &self.coordinates[point.y as usize][point.x as usize] {
                        continue 'target;
                    }
                }

                intersections.push(asteroid_target.clone());
            }

            ast_counts.insert(asteroid.clone(), intersections.len() as i64);
        }

        ast_counts
    }
}
