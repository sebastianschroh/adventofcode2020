use std::fs;


fn main() {

    let filename = "../files/day9-1.txt";
    let contents = read_puzzle(filename);
    let mut numbers: Vec<i64> = Vec::new();
    let preamble: usize = 5;
    // convert contents into a vector of 64-bit integers
    for line in contents.iter(){
        numbers.push(line.parse::<i64>().unwrap());
    }

    println!("{} violates the property!", find_violating_number(&mut numbers, preamble));


}

fn read_puzzle(input: &str) -> Vec<String> {
    let contents = fs::read_to_string(input)
    .expect("Something went wrong with the file");
    let contents: Vec<String> = contents.split("\r\n").map(|x| x.to_string()).collect();
    return contents;
}

fn find_violating_number(numbers: &mut Vec<i64>, preamble: usize) -> i64{
    let mut violation: i64 = 0;
    let mut start: usize = 0;
    let mut violates = true;
    for i in preamble .. numbers.len() -1 {
        println!("{}, {}", &numbers[i], i);
        for number in &numbers[start .. i]{
            for other in &numbers[start .. i] {
                if number + other == numbers[i] {
                    violates = false;
                    break;
                }
            }
            if violates {
                violation = *number;
                break;
            }
        }
        start += 5;
        if violates {
            break;
        }
    }
    return violation

}