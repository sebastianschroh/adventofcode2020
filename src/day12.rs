use std::fs;
use std::time;

struct Waypoint
{
    n: i32,
    e: i32,
}

struct Ferry {
    facing: i32,
    // waypoint is in relation to the ship
    waypoint: Waypoint,
    n: i32,
    e: i32,
}

impl Ferry {

    fn new() -> Ferry {
        Ferry {
            facing: 90,
            waypoint: Waypoint {n: 1, e: 10},
            n: 0,
            e: 0,
        }
    }

    fn read_direction(&mut self, direction: (char,i32)){
        match direction.0 {
            'N' | 'S' => { self.n += direction.1},
            'E' | 'W' => { self.e += direction.1},
            'L' | 'R'=> { self.turn(direction.1)},
                  'F' => { self.move_forward(direction.1)},
             _ => { panic!("unknown direction!");},
        }
    }

    fn move_forward(&mut self, value: i32){
        match self.facing {
            0 => self.n += value,
            90 => self.e += value,
            180 => self.n -= value,
            270 => self.e -= value,
            _ => panic!("unknown facing direction"),
        }
    }

    fn turn(&mut self, degrees: i32){
        let turn = self.facing + degrees;
        if turn < 0 {
            self.facing = 360 + turn;
        }
        else {
            self.facing = turn % 360;
        }
    }

    fn read_direction_waypoint(&mut self, direction: (char, i32)){
        match direction.0 {
            'N' | 'S' => { self.waypoint.n += direction.1},
            'E' | 'W' => { self.waypoint.e += direction.1},
            'L' | 'R' => { self.rotate_waypoint(direction.1);},
                  'F' => { self.move_torward_waypoint(direction.1)},
             _ => { panic!("unknown direction!");},
        }
    }

    fn move_torward_waypoint(&mut self,value: i32){
        let up_down_movement = self.waypoint.n * value;
        let left_right_movement = self.waypoint.e * value;
        self.n += up_down_movement;
        self.e += left_right_movement;
    }

    fn rotate_waypoint(&mut self, degrees: i32) -> (i32, i32){
        let temp = self.waypoint.n;
        match degrees {
            -270 | 90 => {
                    self.waypoint.n = self.waypoint.e * -1;
                    self.waypoint.e = temp;

            },
            270 | -90 => {
                    self.waypoint.n = self.waypoint.e;
                    self.waypoint.e = temp * -1;
            },
            180 | -180 =>{
                self.waypoint.e = self.waypoint.e * -1;
                self.waypoint.n = temp * -1;
            },
            _ =>{
                panic!("unknown rotation of waypoint!");
            },
        }
        (self.waypoint.n, self.waypoint.e)
    }

    fn calculaute_manhattan_distance(&self) -> i32 {
        let mut n = self.n;
        let mut e = self.e;
        if n < 0 {
            n = n * -1;
        }
        if e < 0 {
            e = e * -1;
        }
        n + e
    }

    fn print_location(&self){
        let mut up_down = "North";
        let mut left_right = "East";
        let facing;
        let mut up_down_value = self.n;
        let mut left_right_value = self.e;
        if self.n < 0 {
            up_down = "South";
            up_down_value = up_down_value * -1;
        }
        if self.e < 0 {
            left_right = "West";
            left_right_value = left_right_value * -1;
        }
        match self.facing {
            0 => facing = "North",
            90 => facing = "East",
            180 => facing = "South",
            270 => facing = "West",
            _ => panic!("unknown facing direction"),
        }

        println!("{}: {} , {}: {}, Facing: {}", up_down, up_down_value, left_right, left_right_value, facing);
    }
}


fn main() {
    let now = time::Instant::now();
    let filename = "../files/day12.txt";
    let contents = read_puzzle(filename);
    let directions = get_directions(contents);

    println!("file read in {} micro sec", now.elapsed().as_micros());
    let now = time::Instant::now();
    evasive_action_manhattan_distance(&directions);
    println!("part 1 in {} micro sec", now.elapsed().as_micros());
    let now = time::Instant::now();
    waypoint_manhattan_distance(&directions);
    println!("part 2 in {} micro sec", now.elapsed().as_micros());
}

fn read_puzzle(input: &str) -> Vec<String> {
    let contents = fs::read_to_string(input)
    .expect("Something went wrong with the file");
    let contents: Vec<String> = contents.split("\r\n").map(|x| x.to_string()).collect();
    return contents
}

fn get_directions(input: Vec<String>) -> Vec<(char,i32)> {
    let mut directions = Vec::new();
    for line in input.iter(){
        let direction = line.chars().next()
        .expect("direction was unexpected");
        let value = line.get(1..).unwrap().parse::<i32>()
        .expect("value was unexpected!");
        if direction == 'S' || direction == 'W'  || direction == 'L'{
            directions.push((direction, -1 * value));
        }
        else {
            directions.push((direction, value));
        }
    }
    directions
}

fn evasive_action_manhattan_distance(directions: &Vec<(char, i32)>){
    let mut ship = Ferry::new();
    //ship.print_location();
    for direction in directions.iter() {
        ship.read_direction(*direction);
    }
    //ship.print_location();
    println!("Manhattan distance of ship {}", ship.calculaute_manhattan_distance());
}

fn waypoint_manhattan_distance(directions: &Vec<(char, i32)>){
    let mut ship = Ferry::new();
    //ship.print_location();
    for direction in directions.iter() {
        ship.read_direction_waypoint(*direction);
    }
    //ship.print_location();
    println!("Manhattan distance of ship {}", ship.calculaute_manhattan_distance());
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_waypoint_rotation_90(){
        let mut ferry = Ferry::new();
        ferry.rotate_waypoint(90);
        assert_eq!((-10, 1), (ferry.waypoint.n, ferry.waypoint.e));
    }

    #[test]
    fn test_waypoint_rotation_180(){
        let mut ferry = Ferry::new();
        ferry.rotate_waypoint(180);
        assert_eq!((-1, -10), (ferry.waypoint.n, ferry.waypoint.e));
    }

    #[test]
    fn test_waypoint_rotation_270(){
        let mut ferry = Ferry::new();
        ferry.rotate_waypoint(270);
        assert_eq!((10, -1), (ferry.waypoint.n, ferry.waypoint.e));
    }

    #[test]
    fn test_waypoint_rotation_neg_90(){
        let mut ferry = Ferry::new();
        ferry.rotate_waypoint(-90);
        assert_eq!((10, -1), (ferry.waypoint.n, ferry.waypoint.e));
    }

    #[test]
    fn test_waypoint_rotation_neg_180(){
        let mut ferry = Ferry::new();
        ferry.rotate_waypoint(-180);
        assert_eq!((-1, -10), (ferry.waypoint.n, ferry.waypoint.e));
    }

    #[test]
    fn test_waypoint_rotation_neg_270(){
        let mut ferry = Ferry::new();
        ferry.rotate_waypoint(-270);
        assert_eq!((-10, 1), (ferry.waypoint.n, ferry.waypoint.e));
    }

    #[test]
    fn test_waypoint_rotation_quadrant2(){
        let mut ferry = Ferry{facing: 0, waypoint: Waypoint{n :-1, e: 1}, n: 0, e: 0 };
        let rot90 = ferry.rotate_waypoint(90);
        ferry = Ferry{facing: 0, waypoint: Waypoint{n :-1, e: 1}, n: 0, e: 0 };
        let rot180 = ferry.rotate_waypoint(180);
        ferry = Ferry{facing: 0, waypoint: Waypoint{n :-1, e: 1}, n: 0, e: 0 };
        let rot270 = ferry.rotate_waypoint(270);

        let expected = ((-1,-1), (1,-1), (1,1));
        let actual = (rot90, rot180, rot270);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_waypoint_rotation_quadrant3(){
        let mut ferry = Ferry{facing: 0, waypoint: Waypoint{n :-1, e: -1}, n: 0, e: 0 };
        let rot90 = ferry.rotate_waypoint(90);
        ferry = Ferry{facing: 0, waypoint: Waypoint{n :-1, e: -1}, n: 0, e: 0 };
        let rot180 = ferry.rotate_waypoint(180);
        ferry = Ferry{facing: 0, waypoint: Waypoint{n :-1, e: -1}, n: 0, e: 0 };
        let rot270 = ferry.rotate_waypoint(270);

        let expected = ((1,-1), (1,1), (-1,1));
        let actual = (rot90, rot180, rot270);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_waypoint_rotation_quadrant4(){
        let mut ferry = Ferry{facing: 0, waypoint: Waypoint{n :1, e: -1}, n: 0, e: 0 };
        let rot90 = ferry.rotate_waypoint(90);
        ferry = Ferry{facing: 0, waypoint: Waypoint{n :1, e: -1}, n: 0, e: 0 };
        let rot180 = ferry.rotate_waypoint(180);
        ferry = Ferry{facing: 0, waypoint: Waypoint{n :1, e: -1}, n: 0, e: 0 };
        let rot270 = ferry.rotate_waypoint(270);

        let expected = ((1,1), (-1,1), (-1,-1));
        let actual = (rot90, rot180, rot270);

        assert_eq!(expected, actual);
    }




}