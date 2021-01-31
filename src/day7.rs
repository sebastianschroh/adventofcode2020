use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct BagInfo {
    color: String,
    amount: i32,
}

impl BagInfo {
    fn new(color: String, amount: i32) -> BagInfo {
        BagInfo{color: color, amount: amount}
    }
}

fn main() {

    let filename = "../files/day7-1.txt";
    let contents = read_puzzle(filename);
    let mut bags = create_bags_map(contents);

    println!("{}", find_outer_bag_sum("shiny gold".to_string(), bags));
}

fn read_puzzle(input: &str) -> Vec<String> {
    let contents = fs::read_to_string(input)
    .expect("Something went wrong with the file");
    let contents: Vec<String> = contents.split("\r\n").map(|x| x.to_string()).collect();
    return contents
}

fn create_bags_map(contents: Vec<String>) -> HashMap<String, Vec<BagInfo>>{
    let mut bags: HashMap<String, Vec<BagInfo>> = HashMap::new();
    for content in contents {
        let mut tokens: Vec<String> = content.split(" bags contain ").map(|x| x.to_string()).collect();
        let mut insidebags: Vec<String> = tokens[1].split(", ").map(|x| x.to_string()).collect();
        let mut inside_bags_info: Vec<BagInfo> = Vec::new();
        for mut insidebag in insidebags {
            if insidebag.contains(".") {
                insidebag.remove(insidebag.len() -1);
            }

            let mut amount: String = insidebag.split("bag").collect();
            amount = insidebag.split("bags").collect();
            let mut t: Vec<String> = amount.split(" ").map(|x| x.to_string()).collect();
            let mut number = 0;
            if !amount.contains("no ") {
                number = t[0].chars().next().unwrap().to_digit(10)
                .expect("that's no number");
                let mut color = t[1].to_string();
                color.push_str(" ");
                color.push_str(&t[2]);
                inside_bags_info.push(BagInfo::new(color, number as i32));
            }
        }

        bags.insert(tokens.remove(0), inside_bags_info);
    }
    return bags
}

fn find_outer_bag_sum(color: String, map: HashMap<String, Vec<BagInfo>>) -> i32 {
    let mut sum = 0;
    let color_rec = color;
    for (bag_color, bags) in map {
        find_outer_bag_recursion(color_rec, bags, map);
    }
    return sum
}

fn find_outer_bag_recursion(bag_color: String, bags: Vec<BagInfo>, map: HashMap<String, Vec<BagInfo>>) -> bool {
    if bags.is_empty() {
        return false;
    }
    if bags.iter().any(|b| b.color == bag_color) {
        return true; 
    }
    else {
        for bag in bags {
            let rec_bags = map.get(&bag.color).unwrap();
            return find_outer_bag_recursion(bag.color, rec_bags.to_vec(), map);
        }
        return false;
    }
}