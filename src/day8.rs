use std::fs;

struct Code{
    instr: String,
    value: i32,
}

impl Code {
    fn set_instr(&mut self, instr: String) {
        self.instr = instr;
    }
}



fn main() {

    let filename = "../files/day8.txt";
    let contents = read_puzzle(filename);
    let mut program: Vec<Code> = get_program(contents);

    println!("{}", fix_infinite_loop(program));
}

fn read_puzzle(input: &str) -> Vec<String> {
    let contents = fs::read_to_string(input)
    .expect("Something went wrong with the file");
    let contents: Vec<String> = contents.split("\r\n").map(|x| x.to_string()).collect();
    return contents;
}

fn get_program(lines: Vec<String>) -> Vec<Code> {
    let mut program: Vec<Code> = Vec::new();
    for line in lines.iter() {
        let mut split: Vec<String> = line.split(" ").map(|x| x.to_string()).collect();
        let sign = split[1].remove(0);
        let mut number = split[1].parse::<i32>().unwrap();
        match sign {
            '-' => number = -1 * number,
            '+' => number = 1 * number,
            _ => panic!("unexpected character"),
        }
        let code: Code = Code{instr: split[0].to_string(), value: number};
        program.push(code);
    }

    return program
}

fn fix_infinite_loop(program: Vec<Code>) -> i32 {
    let mut accumulator = 0;
    let mut visited_vec = vec![false; program.len()];
    let mut index = 0; 
    let end = program.len() - 1;

    for instr in program {

        match instr.instr.as_str() {
            "nop" => instr.set_instr("jmp".to_string()),
            "jmp" => instr.set_instr("nop".to_string()),
            "acc" => continue,
            _ => panic!("instruction isn't recognized"),
        }

        while index < end {
            if visited_vec[index] == true {
                accumulator = 0;
                break;
            }

            let code = program[index];
            println!("Code: {}, Instr: {}", code.instr, code.value);
            let instruction = code.instr.to_string();
            visited_vec[index] = true;

            match instruction.as_str() {
                "nop" => index += 1,
                "acc" => {
                        accumulator += code.value;
                        index += 1},
                "jmp" => index = (index as i32 + code.value) as usize,
                _ => panic!("instructions isn't recognized"),
            }
        }
        if index == end {
            break;
        }
        else {
            index = 0;
            match instr.instr.as_str() {
                "nop" => instr.set_instr("jmp".to_string()),
                "jmp" => instr.set_instr("nop".to_string()),
                "acc" => continue,
                _ => panic!("instruction isn't recognized"),
            }
        }
    }
    return accumulator;
}

