use std::fs;
use std::cmp::Ordering;


fn main() {

    let filename = "../files/day9.txt";
    let contents = read_puzzle(filename);
    let mut numbers: Vec<i64> = Vec::new();
    let preamble: usize = 25;
    // convert contents into a vector of 64-bit integers
    for line in contents.iter(){
        numbers.push(line.parse::<i64>().unwrap());
    }
    let violating_number = find_violating_number(&mut numbers, preamble);
    println!("{} violates the property!", violating_number);
    let encryption_weakness = find_contiguous_set_sum(&mut numbers, violating_number);
    println!("{} is the encryption weakness!", encryption_weakness);

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
    for i in preamble .. numbers.len() -1 {
        let mut violates = true;
        //println!("{}, {}", &numbers[i], i);
        for number in &numbers[start .. i]{
            if number > &numbers[i] {
                continue;
            }
            for other in &numbers[start .. i] {
                if other > &numbers[i] {
                    continue;
                }
                //println!("{}, {}, {}, {}", numbers[i], number, other, number + other);
                if number + other == numbers[i] {
                    violates = false;
                    break;
                }
            }

        }
        start = start + 1;
        if violates {
            violation = numbers[i];
            break;
        }
    }
    return violation

}

fn find_contiguous_set_sum(numbers: &mut Vec<i64>, sum: i64) -> i64 {
    let mut contiguous: Vec<i64> = Vec::new();
    let mut found = false;
    for i in 0 .. numbers.len() -1 {
        if numbers[i] == sum {
            continue;
        }
        for number in &numbers[i .. numbers.len()]{
            let contiguous_sum: i64 = contiguous.iter().sum();

            match contiguous_sum.cmp(&sum) {
                Ordering::Less => contiguous.push(*number),
                Ordering::Greater => {
                    contiguous = Vec::new();
                    break;
                },
                Ordering::Equal => {
                    found = true;
                    break;
                },
            }

        }
        if found {
            break;
        }
    }
    let min = contiguous.iter().min().unwrap();
    let max = contiguous.iter().max().unwrap();
    let weakness = min + max;
    return weakness
}