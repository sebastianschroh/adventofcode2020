use std::fs;
use std::time;

fn main() {
    let now = time::Instant::now();
    let filename = "./files/day13-jack.txt";
    let contents = read_puzzle(filename);
    let contents2 = contents.clone();
    let (time, buses) = read_bus(contents);
    let (time2, buses2) = read_bus_pt2(contents2);
    
    println!("file read in {} micro sec", now.elapsed().as_micros());
    let now = time::Instant::now();
    find_earliest_bus_time(time, buses);
    println!("part 1 in {} micro sec", now.elapsed().as_micros());
    let now = time::Instant::now();
    find_earliest_bus_time_pattern(time2, buses2);
    println!("part 2 in {} micro sec", now.elapsed().as_micros());
}

fn read_puzzle(input: &str) -> Vec<String> {
    let contents = fs::read_to_string(input)
    .expect("Something went wrong with the file");
    let contents: Vec<String> = contents.split("\r\n").map(|x| x.to_string()).collect();
    return contents
}

fn read_bus(input: Vec<String>)-> (i32, Vec<i32>){
    let time = input[0].parse::<i32>().unwrap();
    let buses: Vec<i32> = input[1].split(",").filter(|&s| s != "x").map(|x| x.parse::<i32>().unwrap()).collect();
    (time, buses)
}

fn read_bus_pt2(input: Vec<String>) -> (i32, Vec<String>){
    let time = input[0].parse::<i32>().unwrap();
    let buses: Vec<String> = input[1].split(",").map(|x| x.to_string()).collect();
    (time, buses)
}

fn find_earliest_bus_time(time: i32, buses: Vec<i32>){
    let mut earliest = (buses[0], buses[0] - time % buses[0]);
    for bus in buses.iter().skip(1) {
        let difference = bus - time % bus;

        if difference < earliest.1 {
            earliest = (*bus, difference);
        }
    }
    println!("You will take bus {} and arrive at {}. The answer is: {}.", earliest.0, earliest.1 + time, earliest.0 * earliest.1);
}

fn inverse_mod(a: i128, m: i128 ) -> i128{
    for i in 1..m {
        if (((a%m) * (i%m)) % m) == 1 {
            return i
        }
    }
    -1
}



fn find_earliest_bus_time_pattern(time: i32, buses: Vec<String>){
    let m: i128 = buses.iter().filter(|&s| s != "x").map(|x| x.parse::<i128>().unwrap()).product();
    let mut m_i: Vec<i128> = Vec::new();
    let mut r_i: Vec<i128> = Vec::new();
    let mut a_i: Vec<i128> = Vec::new();

    // collect modulo for each input
    for i in 0.. buses.len() {
        if i == 0{
            r_i.push(i as i128);
        }
        else {
            if buses[i] != "x"{
                r_i.push(buses[i].parse::<i128>().unwrap() - i as i128);
            }
        }
    }

    println!("{:?}", r_i);

    // collect m1, ... , mi values
    for i in 0..buses.len() {
        if buses[i] != "x"{
            m_i.push(m / buses[i].parse::<i128>().unwrap());
        }
    }

    println!("{:?}", m_i);

    // collect inverse modulos
    let mut m_index = 0;
    for i in 0..buses.len() {
        if buses[i] != "x"{
            a_i.push(inverse_mod(m_i[m_index], buses[i].parse::<i128>().unwrap()));
            m_index += 1;
        }
    }
    let mut sum = 0;
    for i in 0.. m_i.len() {
        sum += m_i[i] * r_i[i] * a_i[i];
    }
    let answer = sum % m;
    println!("{:?}", a_i);

    println!("{}", answer);
}