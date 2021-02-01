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

    let filename = "../files/day7.txt";
    let contents = read_puzzle(filename);
    let mut bags = create_bags_map(contents);

    println!("{}", find_amount_of_bags(&mut "shiny gold".to_string(), &mut bags) - 1);
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

fn find_outer_bag_sum(desired_color: &String, map: &HashMap<String, Vec<BagInfo>>) -> i32 {
    let mut correct_bags: Vec<String> = Vec::new();
    for (bag_color, bags) in map {
        find_outer_bag_recursion(desired_color, bag_color, bags, map, &mut correct_bags);
    }
    correct_bags.dedup();
    println!("{:?}", correct_bags);
    return correct_bags.len() as i32
}

fn find_outer_bag_recursion(desired_color: &String, bag_color: &String, bags: &Vec<BagInfo>, map: &HashMap<String, Vec<BagInfo>>, correct_bags: &mut Vec<String>){
    if bags.is_empty() {
        
        return;
    }
    if bags.iter().any(|b| b.color == *desired_color) {
        correct_bags.push(bag_color.to_string());
        return;
    }
    else {
        for bag in bags {
            let rec_bags = map.get(&bag.color).unwrap();
            find_outer_bag_recursion(desired_color, bag_color, rec_bags , map, correct_bags);
        }
    }
}

fn find_amount_of_bags(desired_color: &String, map: &HashMap<String, Vec<BagInfo>>) -> i32 {
    let mut sum = 1;
    let inner_bags = map.get(desired_color).unwrap();
    println!("{}", desired_color);
    println!("{:?}", inner_bags);
    for bag in inner_bags{
        sum += bag.amount * find_amount_of_bags(&bag.color, map);
    }

    return sum

}