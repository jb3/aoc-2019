use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64
}

#[derive(Debug)]
pub struct Area {
    grid: HashMap<Point, Tile>,
    score_board: i64,
    ball_pos: Point,
    paddle_pos: Point
}

impl Tile {
    pub fn from_int(i: i64) -> Tile {
        match i {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => panic!("Unimplemented tile type: {}", i)
        }
    }
}

impl Area {
    pub fn new() -> Area {
        Area {
            grid: HashMap::new(),
            score_board: 0,
            ball_pos: Point { x: 0, y: 0 },
            paddle_pos: Point { x: 0, y: 0 }
        }
    }

    pub fn get_joystick(&mut self) -> i64 {
        if self.paddle_pos.x < self.ball_pos.x {
            1
        } else if self.paddle_pos.x > self.ball_pos.x {
            -1
        } else {
            0
        }
    }

    pub fn set_score(&mut self, score: i64) {
        self.score_board = score;
    }

    pub fn set(&mut self, point: Point, tile: Tile) {
        if tile == Tile::Ball {
            self.ball_pos = point.clone();
        }

        if tile == Tile::HorizontalPaddle {
            self.paddle_pos = point.clone();
        }

        self.grid.insert(point, tile);
    }

    pub fn find_count_of(&self, tile: Tile) -> i64 {
        let mut data: Vec<_> = self.grid.iter().collect();

        data.drain_filter(|x| *x.1 == tile).collect::<Vec<_>>().len() as i64
    }

    pub fn print_score(&self) {
        println!("Part 2: {}", self.score_board);
    }
}
