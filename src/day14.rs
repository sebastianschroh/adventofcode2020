use std::fs;
use std::time;
use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    UpdateMaskV2 {
        x_bits: Vec<u64>,
        zeros_mask: u64,
        ones_mask: u64,
    },
    UpdateMask {
        zeros_mask: u64,
        ones_mask: u64,
    }
    ,
    UpdateMemory {
        address: u64,
        value: u64,
    },
}

fn main() {
    let now = time::Instant::now();
    let filename = "./files/day14.txt";
    let contents = read_puzzle(filename);
    let instructions = parse_into_instructions(contents.clone());
    let instructions2 = parse_into_instructions_v2(contents);
    
    println!("file read in {} micro sec", now.elapsed().as_micros());
    let now = time::Instant::now();
    //execute_initialization_instructions(instructions);
    println!("part 1 in {} micro sec", now.elapsed().as_micros());
    let now = time::Instant::now();
    execute_initialization_instructions_v2(instructions2);
    println!("part 2 in {} micro sec", now.elapsed().as_micros());
}

fn read_puzzle(input: &str) -> Vec<String> {
    let contents = fs::read_to_string(input)
    .expect("Something went wrong with the file");
    let contents: Vec<String> = contents.split("\r\n").map(|x| x.to_string()).collect();
    return contents
}

fn parse_into_instructions(contents: Vec<String>) -> Vec<Instruction> {
    let mut parse: Vec<Instruction> = Vec::new();
    for line in contents.iter() {
        let split = line.split(" = ").map(|x| x.to_string()).collect::<Vec<String>>();
        if split[0].contains("mask") {
            parse.push(parse_mask_instruction(&split[1]));
        }
        else if split[0].contains("mem"){
            parse.push(parse_mem_instruction(&split[0], &split[1]));
        }
        else {
            panic!("unknown instruction!");
        }
    }
    parse
}

fn parse_mem_instruction(instr: &String, number: &String) -> Instruction{
    let address = instr.split(&['[', ']'][..]).filter(|&x| x.parse::<u64>().is_ok()).collect::<Vec<&str>>();
    let address = address[0].to_string().parse::<u64>()
    .expect("This is not a valid address number");
    let value = number.parse::<u64>()
    .expect("This is not a valid decimal number");

    Instruction::UpdateMemory{ address: address, value: value}
}

fn parse_mask_instruction(number: &String) -> Instruction {
    let ones_mask_str = number.replace("X", "0");
    let zeros_mask_str = number.replace("1", "X").replace("0", "1").replace("X", "0");

    let ones_mask = u64::from_str_radix(&ones_mask_str[..], 2)
    .expect("Didn't get a proper binary string");
    let zeros_mask = u64::from_str_radix(&zeros_mask_str[..], 2)
    .expect("Didn't get a proper binary string");

    Instruction::UpdateMask{zeros_mask: zeros_mask, ones_mask: ones_mask }
}

fn execute_initialization_instructions(instructions: Vec<Instruction>) {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = &instructions[0];
    for instr in instructions.iter().skip(1) {
        match instr {
            Instruction::UpdateMask {zeros_mask: _, ones_mask: _} => current_mask = &instr,
            Instruction::UpdateMemory{address, value} => update_memory(&current_mask, address, value,  &mut memory),
            _ => panic!("Unsupported instruction for v1"),
        }
    }
    let sum: u64 = memory.values().sum();
    println!("Sum in memory: {}", sum);
}

fn update_memory(mask: &Instruction, address: &u64, value: &u64, memory: &mut HashMap<u64, u64>){
    let ones_mask: u64;
    let zeros_mask: u64;

    match mask {
        Instruction::UpdateMask{zeros_mask: zero, ones_mask: one} => {
            ones_mask = *one;
            zeros_mask = *zero;
        },
        _ => panic!("Unsupported instruction!"),
    }

    let new_value = (*value & (!ones_mask)) | ones_mask;
    let new_value = new_value & !zeros_mask;

    memory.insert(*address, new_value);
}

// part 2

fn execute_initialization_instructions_v2(instructions: Vec<Instruction>) {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = &instructions[0];
    for instr in instructions.iter().skip(1) {
        match instr {
            Instruction::UpdateMaskV2 {x_bits: _, zeros_mask: _, ones_mask: _} => current_mask = &instr,
            Instruction::UpdateMemory{address, value} => update_memory_v2(&current_mask, address, value,  &mut memory),
            _=> panic!("Unsupported instruction for v2"),
        }
    }
    let sum: u64 = memory.values().sum();
    println!("Sum in memory: {}", sum);
}

fn parse_into_instructions_v2(contents: Vec<String>) -> Vec<Instruction> {
    let mut parse: Vec<Instruction> = Vec::new();
    for line in contents.iter() {
        let split = line.split(" = ").map(|x| x.to_string()).collect::<Vec<String>>();
        if split[0].contains("mask") {
            parse.push(parse_mask_instruction_v2(&split[1]));
        }
        else if split[0].contains("mem"){
            parse.push(parse_mem_instruction(&split[0], &split[1]));
        }
        else {
            panic!("unknown instruction!");
        }
    }
    parse
}

fn parse_mask_instruction_v2(number: &String) -> Instruction {
    let ones_mask_str = number.replace("X", "0");
    let zeros_mask_str = number.replace("1", "X").replace("0", "1").replace("X", "0");
    let x_s: String = number.chars().rev().collect();

    let mut x_bits = Vec::new();
    let mut index = 0;
    for c in x_s.chars() {
        if c == 'X'{
            x_bits.push(index);
        }
        index += 1;
    }

    let ones_mask = u64::from_str_radix(&ones_mask_str[..], 2)
    .expect("Didn't get a proper binary string");
    let zeros_mask = u64::from_str_radix(&zeros_mask_str[..], 2)
    .expect("Didn't get a proper binary string");

    Instruction::UpdateMaskV2{x_bits: x_bits, zeros_mask: zeros_mask, ones_mask: ones_mask }
}

fn update_memory_v2(mask: &Instruction, address: &u64, value: &u64, memory: &mut HashMap<u64, u64>){
    let ones_mask: u64;
    let zeros_mask: u64;
    let x_bits: Vec<u64>;
    let mut addresses_to_update: Vec<u64> = Vec::new();

    match mask {
        Instruction::UpdateMaskV2{x_bits: x, zeros_mask: zero, ones_mask: one} => {
            ones_mask = *one;
            zeros_mask = *zero;
            x_bits = x.to_vec();
        },
        _ => panic!("Unsupported instruction!"),
    }

    // applying bit mask first
    let new_value = (*address & (!ones_mask)) | ones_mask;
    
    addresses_to_update.push(new_value);
    // find all permutations based on x_bits
    recursive_floating(&mut addresses_to_update, &x_bits, new_value, 0);
    addresses_to_update.sort();
    addresses_to_update.dedup();
    for address in addresses_to_update {
        memory.insert(address, *value);
    }
}

fn recursive_floating(addresses: &mut Vec<u64>, floating_bits: &Vec<u64>, new_value: u64, index: usize) {
    // insert two into new_value
    if index > floating_bits.len() - 1 {
        return;
    }
    
    let one_value = 1 << floating_bits[index];
    let zero_value = !(1 << floating_bits[index]);

    let new_value_with_one = new_value | one_value;
    let new_value_with_zero = new_value & zero_value;

    addresses.push(new_value_with_one);
    addresses.push(new_value_with_zero);

    recursive_floating(addresses, floating_bits, new_value_with_one, index + 1);
    recursive_floating(addresses, floating_bits, new_value_with_zero, index + 1);

}

