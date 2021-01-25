use std::fs;

fn main() {

    
    let numbers = fs::read_to_string("../files/day1.txt")
        .expect("Something went wrong with the file");
    let numbers = numbers.split("\r\n");
    let numbers: Vec<i32> = numbers.map(|x| x.parse::<i32>().unwrap()).collect();

    let length = numbers.len();
    let mut answer = -1;

    for i in 0 .. length{
        for j in i + 1 ..  length{
            for k in j + 1 .. length {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    answer = numbers[i] * numbers[j] * numbers[k];
                }
            }
        }
    }

    println!("Hello, world! {}", answer);
}
