use std::fs;


fn main() {

    let filename = "../files/day11.txt";
    let mut contents = read_puzzle(filename);
    let mut contents2 = contents.clone();
    find_occupied_seats_pt1(&mut contents);
    find_occupied_seats_pt2(&mut contents2);
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

    let l = 0;
    let r = 1;
    let t = 2;
    let tl = 3;
    let tr = 4;
    let b = 5;
    let bl = 6;
    let br = 7;

    loop {
        for index in 0..contents.len(){
            for jndex in 0..contents[index].len(){
                let mut directions = ['.'; 8];
                if jndex != 0 {
                    directions[l] = *contents[index].get(jndex - 1).unwrap();
                }
                if index != 0 {
                    directions[t] = contents[index - 1][jndex];
                    directions[tr] = *contents[index - 1].get(jndex + 1).unwrap_or(&'.');
                    if jndex != 0 {
                        directions[tl] = *contents[index - 1].get(jndex - 1).unwrap();
                    }
                }
                if index != contents.len() - 1{
                    directions[b] = contents[index + 1][jndex];
                    directions[br] = *contents[index + 1].get(jndex + 1).unwrap_or(&'.');
                    if jndex != 0 {
                        directions[bl] = *contents[index + 1].get(jndex - 1).unwrap();;
                    }
                }

                directions[r] = *contents[index].get(jndex + 1).unwrap_or(&'.');


                if contents[index][jndex] == 'L' && not_adjacent(&directions){
                    copy[index][jndex] = '#';
                }
                else if contents[index][jndex] == '#' && four_or_more_adjacent(&directions){
                    copy[index][jndex] = 'L';
                }
            }
        }

        if matrix_is_equal(&contents, &copy) {
            break;
        }
        *contents = copy.clone();
    }

    println!("{}", amount_of_occupied_seats(&contents));
}

fn find_occupied_seats_pt2(contents: &mut Vec<Vec<char>>){
    let mut copy = contents.clone();

    let l = 0;
    let r = 1;
    let t = 2;
    let tl = 3;
    let tr = 4;
    let b = 5;
    let bl = 6;
    let br = 7;

    loop {
        for index in 0..contents.len(){
            for jndex in 0..contents[index].len(){
                let mut directions = ['.'; 8];
                if jndex != 0 {
                    directions[l] = *contents[index].get(jndex - 1).unwrap();
                }
                if index != 0 {
                    directions[t] = contents[index - 1][jndex];
                    directions[tr] = *contents[index - 1].get(jndex + 1).unwrap_or(&'.');
                    if jndex != 0 {
                        directions[tl] = *contents[index - 1].get(jndex - 1).unwrap();
                    }
                }
                if index != contents.len() - 1{
                    directions[b] = contents[index + 1][jndex];
                    directions[br] = *contents[index + 1].get(jndex + 1).unwrap_or(&'.');
                    if jndex != 0 {
                        directions[bl] = *contents[index + 1].get(jndex - 1).unwrap();;
                    }
                }

                directions[r] = *contents[index].get(jndex + 1).unwrap_or(&'.');


                if contents[index][jndex] == 'L' && not_adjacent(&directions){
                    copy[index][jndex] = '#';
                }
                else if contents[index][jndex] == '#' && four_or_more_adjacent(&directions){
                    copy[index][jndex] = 'L';
                }
            }
        }

        if matrix_is_equal(&contents, &copy) {
            break;
        }
        *contents = copy.clone();
    }

    println!("{}", amount_of_occupied_seats(&contents));
}

fn in_grid(row_size: usize, col_size: usize, row_index: isize, col_index: isize) -> bool{
    if row_index < 0 || col_index < 0{
        false
    }
    let row_index = row_index as usize;
    let col_index = col_index as usize;
    if row_index > row_size || col_index > col_size {
        false
    }
    true

}

fn not_adjacent(directions: &[char]) -> bool{
    !directions.contains(&'#')
}

fn four_or_more_adjacent(directions: &[char]) -> bool{
    let mut sum = 0;
    for d in directions.iter(){
        if *d == '#' {
            sum += 1;
        }
    }
    sum >= 4
}

fn matrix_is_equal(mat1: &Vec<Vec<char>>, mat2: &Vec<Vec<char>>) -> bool {
    mat1.iter()
    .zip(mat2)
    .all(|(a,b)| a.iter().zip(b).all(|(x,y)| *x == *y))
}

fn amount_of_occupied_seats(mat1: &Vec<Vec<char>>) -> usize {
    mat1.iter().map(|v| v.iter().filter(|&c| *c == '#').count()).sum()
}