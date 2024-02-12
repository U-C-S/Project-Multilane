use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");

    let two_lane_bidir_road = RoadLaned {
        lanes: VecDeque::from([
            Lane {
                id: 1,
                length: None,
                speed_limit: 60,
                direction: Direction::Left,
            },
            Lane {
                id: 2,
                length: None,
                speed_limit: 60,
                direction: Direction::Right,
            }
        ]),
        median_strip: false,
    };
}

struct Lane {
    id: i32,
    length: Option<i32>,
    // width: i32,
    speed_limit: i32,
    direction: Direction,
}

enum Direction {
    Left,
    Right,
}

struct RoadLaned {
    lanes: VecDeque<Lane>,
    median_strip: bool,
}

struct Node {
    path: String, // quadratic bezier curve serialized as a string
}