use std::fs::File;
use std::io::Read;

use std::collections::HashSet;

mod moon;

use moon::{Moon, Vector3};

fn gcd(x: i64, y: i64) -> i64 {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

fn lcm(x: i64, y: i64) -> i64 {
    if x == 0 || y == 0 {
        0
    } else {
        (x * y) / gcd(x, y)
    }
}

fn main() {
    let mut moons = get_input();

    let mut i = 0;
    let target = 1000;

    let mut prev_x = HashSet::new();
    let mut prev_y = HashSet::new();
    let mut prev_z = HashSet::new();

    let (mut step_x, mut step_y, mut step_z) = (None, None, None);

    let mut energy = 0;

    loop {
        for moon1 in 0..moons.len() {
            for moon2 in 0..moons.len() {
                if moons[moon1] == moons[moon2] {
                    continue;
                }

                if moons[moon1].location.x > moons[moon2].location.x {
                    moons[moon1].velocity += Vector3 {
                        x: -1,
                        y: 0,
                        z: 0
                    }
                }

                if moons[moon1].location.x < moons[moon2].location.x {
                    moons[moon1].velocity += Vector3 {
                        x: 1,
                        y: 0,
                        z: 0
                    }
                }

                if moons[moon1].location.y > moons[moon2].location.y {
                    moons[moon1].velocity += Vector3 {
                        x: 0,
                        y: -1,
                        z: 0
                    }
                }

                if moons[moon1].location.y < moons[moon2].location.y {
                    moons[moon1].velocity += Vector3 {
                        x: 0,
                        y: 1,
                        z: 0
                    }
                }

                if moons[moon1].location.z > moons[moon2].location.z {
                    moons[moon1].velocity += Vector3 {
                        x: 0,
                        y: 0,
                        z: -1
                    }
                }

                if moons[moon1].location.z < moons[moon2].location.z {
                    moons[moon1].velocity += Vector3 {
                        x: 0,
                        y: 0,
                        z: 1
                    }
                }
            }
        }

        for moon in &mut moons {
            moon.apply_velocity();
        }

        let mx: Vec<_> = moons.iter().map(|m| (m.location.x, m.velocity.x)).collect();

        let my: Vec<_> = moons.iter().map(|m| (m.location.y, m.velocity.y)).collect();

        let mz: Vec<_> = moons.iter().map(|m| (m.location.z, m.velocity.z)).collect();

        if !prev_x.insert(mx) && step_x.is_none() {
            step_x = Some(i)
        }

        if !prev_y.insert(my) && step_y.is_none() {
            step_y = Some(i)
        }

        if !prev_z.insert(mz) && step_z.is_none() {
            step_z = Some(i)
        }

        if step_x.is_some() && step_y.is_some() && step_z.is_some() {
            break;
        }

        i += 1;

        if i == target {
            for moon in &moons {
                let pot = moon.location.x.abs() + moon.location.y.abs() + moon.location.z.abs();
                let kin = moon.velocity.x.abs() + moon.velocity.y.abs() + moon.velocity.z.abs();
                energy += pot * kin;
            }
        }
    }

    println!("Part 1: {}", energy);

    let (step_x, step_y, step_z) = (step_x.unwrap(), step_y.unwrap(), step_z.unwrap());
    println!("Part 2: {}", lcm(lcm(step_x, step_y), step_z))
}

fn get_input() -> Vec<Moon> {
    let mut f = File::open("../input").unwrap();

    let mut b = String::new();

    f.read_to_string(&mut b).unwrap();

    let moons = b.split_terminator('\n').map(|x| Moon::parse(x.to_string())).collect::<Vec<Moon>>();

    moons
}
