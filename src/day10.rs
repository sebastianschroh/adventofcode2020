use std::fs;
use std::collections::HashMap;

fn main() {

    let filename = "../files/day10.txt";
    let mut contents = read_puzzle(filename);
    contents.sort();

    find_adaptor_differences(&mut contents);
    find_total_adaptor_combination(&contents);

}

fn read_puzzle(input: &str) -> Vec<u16> {
    let contents = fs::read_to_string(input)
    .expect("Something went wrong with the file");
    let contents: Vec<u16> = contents.split("\r\n").map(|x| x.parse::<u16>().unwrap()).collect();
    return contents;
}

fn find_adaptor_differences(contents: &mut Vec<u16>){
    contents.sort();
    let vector: Vec<u16> = contents.windows(2).map(|slice| slice[1]-slice[0]).collect();
    let difference_by_3 = vector.iter().filter(|&n| *n == 3).count() + 1;
    let difference_by_1 = vector.iter().filter(|&n| *n == 1).count() + 1;
    println!("Adaptors differing by 1: {}", difference_by_1);
    println!("Adaptors differing by 3: {}", difference_by_3);
    println!("Differences multipled: {}", difference_by_3 * difference_by_1);
}

fn find_total_adaptor_combination(contents: &[u16]){
    let mut number_of_adaptors: HashMap<u16, u128> = HashMap::new();
    number_of_adaptors.insert(0, 1);
    for x in contents.iter() {
        let mut adaptors = 0;
        for i in 1..=3 {
            if (*x as i16 - i) < 0 {
                break;
            }
            adaptors += number_of_adaptors.get(&(x - i as u16)).unwrap_or(&0);
        }
        number_of_adaptors.insert(*x, adaptors);
    }
    println!("{:?}", number_of_adaptors);
    println!("{}", *number_of_adaptors.get(contents.last().unwrap()).unwrap());
}