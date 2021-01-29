use std::fs;

fn main() {

    let filename = "../files/day3.txt";
    let contents = read_tree_map(filename);
    let trees = [ find_trees_in_sloped_path(&contents, 1, 1),
                    find_trees_in_sloped_path(&contents, 3, 1),
                    find_trees_in_sloped_path(&contents, 5, 1),
                    find_trees_in_sloped_path(&contents, 7, 1),
                    find_trees_in_sloped_path(&contents, 1, 2)
    ];
    let mut multiplied: i64 = 1;
    for tree in trees.iter() {
        println!("number of trees: {}", tree);
        multiplied = tree * multiplied;
    }
        println!("multiplied together: {}", multiplied);
}

fn read_tree_map(input: &str) -> Vec<String> {
    let contents = fs::read_to_string(input)
    .expect("Something went wrong with the file");
    let contents: Vec<String> = contents.split("\r\n").map(|x| x.to_string()).collect();
    return contents;
}

fn find_trees_in_sloped_path(tree_map: &Vec<String>, right: usize, down: usize) -> i64 {
    let mut col = 0;
    let mut row = 0;
    let mut counter = 0;
    let length = tree_map.len() - 1;

    while row < length{
        col += right;
        row += down;
        let squares: Vec<char> = tree_map[row].chars().collect();
        if col > squares.len() - 1 {
            col = col - squares.len();
        }
        let square = squares[col];
        if square == '#'{
            counter = counter + 1;
        }
    }
    return counter
}
