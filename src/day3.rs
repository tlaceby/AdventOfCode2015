use std::{collections::HashMap, fs::read_to_string, hash::Hash};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Cordinate {
    x: i64,
    y: i64,
}

// Usefull helper methods. Not needed but nice to use.
impl Cordinate {
    fn new(x: i64, y: i64) -> Cordinate {
        Cordinate { x, y }
    }

    fn goto(&mut self, direction: char) {
        if direction == '^' {
            // UP
            self.y += 1;
        } else if direction == '>' {
            // RIGHT
            self.x += 1;
        } else if direction == 'v' {
            // DOWN
            self.y -= 1;
        } else {
            // LEFT
            self.x -= 1;
        }
    }
}

pub fn perfectly_spherical_houses_in_vacuum() {
    let mut visited = HashMap::new();
    let mut santa_location = Cordinate::new(0, 0);
    let mut robo_santa_location = Cordinate::new(0, 0);
    let mut houses = 0;
    let mut santas_turn = true;

    // Iterate over each direction and start working with them.
    for (_, direction) in read_to_string("./inputs/day3").unwrap().chars().enumerate() {
        // Keeps track of the current location as this altrinates between
        // each iteration.
        let loc: Cordinate;

        // MOve the location of  each person depending on which turn we are on.
        if santas_turn {
            santa_location.goto(direction);
            loc = santa_location;
        } else {
            robo_santa_location.goto(direction);
            loc = robo_santa_location;
        }

        // Incriment count of visited locations
        houses += 1;
        visited
            .entry(loc)
            .and_modify(|count| {
                houses -= 1;
                *count = *count + 1;
            })
            .or_insert(1);

        santas_turn = !santas_turn;
    }

    println!("There were {} lucky houses that recieved presents.", houses);
}
