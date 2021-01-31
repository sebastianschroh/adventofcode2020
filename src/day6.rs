use std::fs;
use std::collections::HashMap;


fn main() {

    let filename = "../files/day6.txt";
    let contents = read_puzzle(filename);
    let sum = find_sum_of_answers_everyone(contents);
    println!("Count: {}", sum);
}

fn read_puzzle(input: &str) -> Vec<String> {
    let contents = fs::read_to_string(input)
    .expect("Something went wrong with the file");
    let contents: Vec<String> = contents.split("\r\n\r\n").map(|x| x.to_string()).collect();
    return contents
}

fn find_sum_of_answers_anyone(group: Vec<String>) ->  i32 {
    let mut sum: i32 = 0;
    for answers in group {
        let answers = answers.replace("\n", "").replace("\r", "");
        let mut y: Vec<char> = answers.chars().collect();
        y.sort();
        y.dedup();
        sum = sum + y.len() as i32;
    }
    return sum
}

fn find_sum_of_answers_everyone(group: Vec<String>) ->  i32 {
    let mut sum: i32 = 0;
    for answers in group {
        let answers: Vec<String> = answers.split("\r\n").map(|x| x.to_string()).collect();
        let count = answers.len();
        let mut map = HashMap::new();
        for answer in answers {
            let chars: Vec<char> = answer.chars().collect();
            for c in chars {
                if map.contains_key(&c) {
                    *map.get_mut(&c).unwrap() += 1;
                } else {

                    map.insert(c, 1);
                }
            }
        }
        let maplength = map.len() as i32;
        for (key, value) in map {
            if value == count {
                sum += 1;
            }
        }
    }
    return sum
}


