use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");

    let two_lane_bidir_road = road_laned {
        lanes: VecDeque::from([
            lane {
                id: 1,
                length: None,
                speed_limit: 60,
                direction: direction::left,
            },
            lane {
                id: 2,
                length: None,
                speed_limit: 60,
                direction: direction::right,
            }
        ]),
        median_strip: false,
    };
}

struct lane {
    id: i32,
    length: Option<i32>,
    // width: i32,
    speed_limit: i32,
    direction: direction,
}

enum direction {
    left,
    right,
}

struct road_laned {
    lanes: VecDeque<lane>,
    median_strip: bool,
}

struct node {
    
}