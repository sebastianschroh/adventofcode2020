use std::fs;
use named_tuple::named_tuple;

named_tuple!(
    #[derive(Clone)]
    struct PasswordAndPolicy{
        letter: char,
        password: String,
        policy1: i32,
        policy2: i32,
    }
);


fn main() {

    let filename = "../files/day2.txt";
    let contents = read_password_policy(filename);
    let pass_policies: Vec<PasswordAndPolicy> = parse_contents_into_password_policies(contents);
    let count = check_for_number_of_char(&pass_policies);
    let count2 = check_for_char_at_indices(&pass_policies);

    println!("{} passwords follow the first policy, {} passwords follow the second policy", count, count2);

}

fn read_password_policy(input: &str) -> Vec<String> {
    let contents = fs::read_to_string(input)
    .expect("Something went wrong with the file");
    let contents: Vec<String> = contents.split("\r\n").map(|x| x.to_string()).collect();
    return contents;
}

fn parse_contents_into_password_policies(contents: Vec<String>) -> Vec<PasswordAndPolicy> {
    let mut pass_policies: Vec<PasswordAndPolicy> = Vec::new();

    for line in contents {
        // split into "min-max letter: password" into "minx max letter" & "password"
        let split: Vec<String> = line.split(":").map(|x| x.to_string()).collect();
        let mut password = String::new();
        password.push_str(split[1].trim());

        // split "min-max letter" into "min-max" & "letter"
        let policy = split[0].trim().split(" ").collect::<Vec<&str>>();
        let letter = policy[1].chars().next()
            .expect("string is empty");

        // split "min-max" into "min" & "max"
        let policy = policy[0].split("-").collect::<Vec<&str>>();
        let policy1 = policy[0].trim().parse::<i32>().unwrap();
        let policy2 = policy[1].trim().parse::<i32>().unwrap();

        let pass_policy: PasswordAndPolicy = (letter, password, policy1, policy2).into();
        pass_policies.push(pass_policy);
    }

    return pass_policies;
}
// part 2
fn check_for_char_at_indices(pass_policies: &Vec<PasswordAndPolicy>) -> i32 {
    let mut counter = 0;
    for pass_policy in pass_policies{
        let char_vec: Vec<char> = pass_policy.password().chars().collect();
        let index1 = *pass_policy.policy1() - 1;
        let index2 = *pass_policy.policy2() - 1;
        let char_at_policy1 = char_vec[index1 as usize] == *pass_policy.letter();
        let char_at_policy2 = char_vec[index2 as usize] == *pass_policy.letter();
        if (char_at_policy1 && !char_at_policy2) || (!char_at_policy1 && char_at_policy2){
            counter = counter + 1;
        } 

    }
    return counter;
}

fn check_for_number_of_char(pass_policies: &Vec<PasswordAndPolicy>) -> i32 {
    let mut counter = 0;
    for pass_policy in pass_policies{
        let mut letter_count = 0;
        let char_vec: Vec<char> = pass_policy.password().chars().collect();

        for c in char_vec{
            if c == *pass_policy.letter()
            {
                letter_count = letter_count + 1;
            }
        }

        if letter_count >= *pass_policy.policy1() && letter_count <= *pass_policy.policy2() {
            counter = counter + 1;
        }
    }
    return counter;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn placeholder (){
        let p: PasswordAndPolicy = ('c', "yeet".to_string(),  0 , 0).into();
        assert!(*p.letter() == 'c');
    }
}
