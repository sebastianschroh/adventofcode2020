use std::fs;

struct Seat {
    row: i32,
    col: i32,
    id: i32,
}

struct Plane {
    row_size: i32,
    col_size: i32,
    seats: Vec<Seat>,
}

#[derive(Debug)]
struct Bounds {
    upper: f32,
    lower: f32,
}

fn main() {

    let filename = "../files/day5.txt";
    let contents = read_puzzle(filename);
    let mut plane: Plane = Plane {row_size: 128, col_size: 8, seats: Vec::new()};
    let mut largest_id = -1;
    for instructions in contents {
        let seat = find_seat((plane.row_size - 1) as f32 , (plane.col_size -1) as f32, instructions);
        println!("{}", seat.id);
        if seat.id > largest_id{
            largest_id = seat.id;
        }
        plane.seats.push(seat);
    }

    let my_seat_id = check_for_my_seat(&mut plane.seats);
    println!("My seat id: {}", my_seat_id);
}

fn read_puzzle(input: &str) -> Vec<String> {
    let contents = fs::read_to_string(input)
    .expect("Something went wrong with the file");
    let contents: Vec<String> = contents.split("\r\n").map(|x| x.to_string()).collect();
    return contents;
}

fn get_lower_half(bounds: &mut Bounds){
    bounds.upper = ((bounds.upper + bounds.lower)/2.0).floor();
}

fn get_upper_half(bounds: &mut Bounds){
    bounds.lower = ((bounds.upper + bounds.lower)/2.0).ceil();
}

fn find_seat(row_size: f32, col_size: f32, instructions: String) -> Seat {

    let mut seat: Seat = Seat{row: -1, col: -1, id: -1};
    let mut row_bounds: Bounds = Bounds{upper: row_size, lower: 0.0};
    let mut col_bounds: Bounds = Bounds{upper: col_size, lower: 0.0};  
    let instructions: Vec<char> = instructions.chars().collect();

    for instruction in instructions {
        match instruction {
            'F' => get_lower_half(&mut row_bounds),
            'B' => get_upper_half(&mut row_bounds),
            'L' => get_lower_half(&mut col_bounds),
            'R' => get_upper_half(&mut col_bounds),
            _ => panic!("unexpected character"),
        }
    };
    seat.row = row_bounds.upper as i32;
    seat.col = col_bounds.upper as i32;
    seat.id = seat.row * 8 + seat.col;
    return seat
}

fn check_for_my_seat(seats: &mut Vec<Seat>) -> i32{
    seats.sort_by(|a , b| a.id.cmp(&b.id));
    let mut prev_id = seats[0].id;
    let mut my_id = -1;
    for seat in seats {
        println!("{}", seat.id);
        if seat.id - prev_id > 1 {
            my_id = seat.id - 1;
        }
        prev_id = seat.id; 
    }

    return my_id
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_get_lower(){
        let mut bounds = Bounds {upper: 127 as f32, lower: 0 as f32};
        get_lower_half(&mut bounds);
        assert_eq!(bounds.upper, 63.0);
    }
    #[test]
    fn test_get_upper(){
        let mut bounds = Bounds {upper: 63 as f32, lower: 0 as f32};
        get_upper_half(&mut bounds);
        assert_eq!(bounds.lower, 32.0);
    }

    #[test]
    fn test_find_seat1(){
        let seat = find_seat(127 as f32, 7 as f32, "BFFFBBFRRR".to_string());
        assert_eq!(seat.id, 567);
    }

    #[test]
    fn test_row_find_seat2(){
        let seat = find_seat(127 as f32, 7 as f32, "FFFBBBFRRR".to_string());
        assert_eq!(seat.id, 119);
    }
    
    #[test]
    fn test_row_find_seat3(){
        let seat = find_seat(127 as f32, 7 as f32, "BBFFBBFRLL".to_string());
        assert_eq!(seat.id, 820);
    }

}


