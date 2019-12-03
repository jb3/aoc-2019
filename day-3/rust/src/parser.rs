use super::wire::{Wire, Point};

pub fn parse(instructions: &str) -> Wire {
    let inst = instructions.split(",").map(|x| x.to_string()).collect::<Vec<String>>();


    let mut next_point = Point::new(0, 0);
    let mut points: Vec<Point> = vec![];

    for instruction in inst {
        let dir = &instruction[..1];
        let dist = &instruction[1..].parse::<i64>()
          .expect(&format!("Could not parse integer of {}", instruction));

        for _ in 0..*dist {
            match dir {
                "U" => next_point.up(1),
                "D" => next_point.down(1),
                "L" => next_point.left(1),
                "R" => next_point.right(1),
                _ => panic!("Unexpected direction: {}", dir)
            };
            points.push(next_point.clone());
        }
    };

    Wire::new(points)
}
