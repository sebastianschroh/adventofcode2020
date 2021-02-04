use std::fs;
use std::time;


fn main() {
    let now = time::Instant::now();
    let filename = "../files/day11.txt";
    let mut contents = read_puzzle(filename);
    println!("file read in {} ms", now.elapsed().as_millis());
    let mut contents2 = contents.clone();
    let now = time::Instant::now();
    find_occupied_seats_pt1(&mut contents);
    println!("part 1 in {} ms", now.elapsed().as_millis());
    let now = time::Instant::now();
    find_occupied_seats_pt2(&mut contents2);
    println!("file read in {} ms", now.elapsed().as_millis());
}

fn read_puzzle(input: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(input)
    .expect("Something went wrong with the file");
    let contents: Vec<String> = contents.split("\r\n").map(|x| x.to_string()).collect();
    let contents: Vec<Vec<char>> = contents.iter().map(|x| x.chars().collect()).collect();
    return contents
}

fn find_occupied_seats_pt1(contents: &mut Vec<Vec<char>>){
    let mut copy = contents.clone();

    loop {
        for index in 0..contents.len(){
            for jndex in 0..contents[index].len(){
                let mut directions = Vec::new();
                if contents[index][jndex] == '.' {
                    continue;
                }

                // check if outside left most column
                if jndex != 0 {
                    directions.push(contents[index][jndex - 1]);
                }
                // check if outside right most column
                if jndex != contents[index].len() - 1 {
                    directions.push(contents[index][jndex + 1]);
                }
                // check if outside top most row
                if index != 0 {
                    directions.push(contents[index - 1][jndex]);
                    if jndex != contents[index].len() - 1 {
                        directions.push(contents[index -1][jndex + 1]);
                    }
                    if jndex != 0 {
                        directions.push(contents[index - 1][jndex - 1]);
                    }
                }
                // check if outside bottom most row
                if index != contents.len() - 1{
                    directions.push(contents[index + 1][jndex]);
                    if jndex != contents[index].len() - 1 {
                        directions.push(contents[index + 1][jndex + 1]);
                    }
                    if jndex != 0 {
                        directions.push(contents[index + 1][jndex - 1]);
                    }
                }


                if contents[index][jndex] == 'L' && not_adjacent(&directions, '#'){
                    copy[index][jndex] = '#';
                }
                else if contents[index][jndex] == '#' && num_of_adjacent(&directions, 4, '#'){
                    copy[index][jndex] = 'L';
                }
            }
        }

        if *contents == copy {
            break;
        }
        *contents = copy.clone();
    }

    println!("{}", amount_of_occupied_seats(&contents));
}

fn find_occupied_seats_pt2(contents: &mut Vec<Vec<char>>){
    let mut copy = contents.clone();

    loop {
        for index in 0..contents.len(){
            for jndex in 0..contents[index].len(){
                let mut directions = Vec::new();

                // check right
                check_directions(&contents, &mut directions, index as isize, jndex as isize, 0, 1);
                // check left
                check_directions(&contents, &mut directions, index as isize, jndex as isize, 0, -1);
                // check up
                check_directions(&contents, &mut directions, index as isize, jndex as isize, -1, 0);
                // check down
                check_directions(&contents, &mut directions, index as isize, jndex as isize, 1, 0);
                // check up and left
                check_directions(&contents, &mut directions, index as isize, jndex as isize, -1, -1);
                // check up and right
                check_directions(&contents, &mut directions, index as isize, jndex as isize, -1, 1);
                // check down and left
                check_directions(&contents, &mut directions, index as isize, jndex as isize, 1, -1);
                // check down and right
                check_directions(&contents, &mut directions, index as isize, jndex as isize, 1, 1);


                if contents[index][jndex] == 'L' && not_adjacent(&directions, '#'){
                    copy[index][jndex] = '#';
                }
                else if contents[index][jndex] == '#' && num_of_adjacent(&directions, 5, '#'){
                    copy[index][jndex] = 'L';
                }
            }
        }
        if *contents == copy {
            break;
        }
        *contents = copy.clone();
    }

    println!("{}", amount_of_occupied_seats(&contents));
}

fn check_directions(mat: &Vec<Vec<char>>, directions: &mut Vec<char>, x: isize, y: isize, dx:  isize, dy:  isize){
    let row_length = (mat.len() - 1) as isize;
    let col_length = (mat[0].len() - 1) as isize;
    let mut x = x;
    let mut y = y;
    loop {
        if x + dx < 0 || y + dy < 0 || x + dx > row_length || y + dy > col_length {
            break;
        }
        let index = (x + dx) as usize;
        let jndex = (y + dy) as usize;
        if mat[index][jndex] == '#' || mat[index][jndex] == 'L' {
            directions.push(mat[index][jndex]);
            break;
        }
        x = x + dx; 
        y = y + dy;
    }
}

fn not_adjacent(directions: &[char], c: char) -> bool{
    !directions.contains(&c)
}

fn num_of_adjacent(directions: &[char], threshold: usize, c: char) -> bool{
    let mut sum = 0;
    for d in directions.iter(){
        if *d == c {
            sum += 1;
        }
    }
    sum >= threshold
}


fn amount_of_occupied_seats(mat1: &Vec<Vec<char>>) -> usize {
    mat1.iter().flatten().filter(|&&c| c == '#').count()
}