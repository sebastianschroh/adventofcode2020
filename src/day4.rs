use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Passport {
    birth: String,
    issue: String,
    expiration: String,
    height: String,
    hair_color: String,
    eye_color: String,
    pid: String,
    cid: String,
}

impl Passport {
    pub fn new() -> Passport {
        Passport {
            birth: "".to_string(),
            issue: "".to_string(),
            expiration: "".to_string(),
            height: "".to_string(),
            hair_color: "".to_string(),
            eye_color: "".to_string(),
            pid: "".to_string(),
            cid: "".to_string(),
        }
    }

    pub fn valid(&self) -> bool {
        let mut valid = true;

        if !self.birth_valid() {
            valid = false;
        }
        else if !self.issue_valid() {
            valid = false;
        }
        else if !self.expiration_valid() {
            valid = false;
        }
        else if !self.height_valid() {
            valid = false;
        }
        else if !self.hair_valid() {
            valid = false;
        }
        else if !self.eye_valid() {
            valid = false;
        }
        else if !self.pid_valid() {
            valid = false;
        }
        return valid

    }

    pub fn birth_valid(&self) -> bool {
        if self.birth == "".to_string() {
            return false
        }
        let year = self.birth.parse::<i32>()
        .expect("birth parse was weird!");
        let valid_length = self.birth.len();
        return valid_length == 4 && (year >= 1920 && year <= 2002)
    }

    pub fn issue_valid(&self) -> bool {
        if self.issue == "".to_string() {
            return false
        }
        let year = self.issue.parse::<i32>()
        .expect("issue parse was weird!");
        let valid_length = self.issue.len();
        return valid_length == 4 && (year >= 2010 && year <= 2020)
    }

    pub fn expiration_valid(&self) -> bool {
        if self.expiration == "".to_string() {
            return false
        }
        let year = self.expiration.parse::<i32>()
        .expect("expiration parse was weird!");
        let valid_length = self.expiration.len();
        return valid_length == 4 && (year >= 2020 && year <= 2030)
    }

    pub fn height_valid(&self) -> bool {
        if self.height == "".to_string() {
            return false
        }
        let length = self.height.len();
        let units = self.height[length - 2 .. length].to_string();
        if units == "cm" {
            let value = self.height[0 .. length - 2].parse::<i32>()
            .expect("height (cm) parse was weird!");
            return value >= 150 && value <= 193
        }
        else if units == "in" {
            let value = self.height[0 .. length - 2].parse::<i32>()
            .expect("height (in) parse was weird!");
            return value >= 59 && value <= 76
        }
        else {
            return false
        }
    }

    pub fn hair_valid(&self) -> bool{
        let length = self.hair_color.len();
        if self.hair_color == "".to_string() || length != 7{
            return false
        }
        let first_char = self.hair_color.chars().next()
        .expect("first char in hair color was weird!");
        let is_alphanumeric = self.hair_color[1..length].to_string().chars().all(|c| c.is_digit(16));
        return first_char == '#' && is_alphanumeric
    }

    pub fn eye_valid(&self) -> bool {
        let length = self.eye_color.len();
        if self.eye_color == "".to_string() || length != 3{
            return false
        }
        match &self.eye_color[..]{
            "amb" => return true,
            "blu" => return true,
            "brn" => return true,
            "gry" => return true,
            "grn" => return true,
            "hzl" => return true,
            "oth" => return true,
            "" => return false,
            _=> return false,
        }
    }

    pub fn pid_valid(&self) -> bool {
        let length = self.pid.len();
        if self.pid == "".to_string() || length != 9 {
            return false
        }
        let is_numeric = self.pid.to_string().chars().all(char::is_numeric);
        return is_numeric

    }
}

fn main() {
    let mut counter = 0;
    let filename = "../files/day4.txt";

    let valid_passports: Vec<Passport> = collect_passport_information(filename);


    for passport in valid_passports{
        if passport.valid() {
            counter = counter + 1;
            println!("{:?}", passport);
        }
    }

    println!("number of valid passport: {}", counter);
}

fn collect_passport_information(filename: &str) -> Vec<Passport> {
    let mut valid_passports: Vec<Passport> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        let mut passport = Passport::new();
        for line in lines {
            if let Ok(information) = line{
                if information == "" {
                    valid_passports.push(passport);
                    passport = Passport::new();
                    continue;
                }
                let information: Vec<String> = information.split(" ").map(|x| x.to_string()).collect();
                for data in information {
                    let length = data.len();
                    let field = &data[0..3];
                    match field {
                        "byr" => passport.birth = data[4..length].to_string(),
                        "iyr" => passport.issue = data[4..length].to_string(),
                        "eyr" => passport.expiration = data[4..length].to_string(),
                        "hgt" => passport.height = data[4..length].to_string(),
                        "hcl" => passport.hair_color = data[4..length].to_string(),
                        "ecl" => passport.eye_color = data[4..length].to_string(),
                        "pid" => passport.pid = data[4..length].to_string(),
                        "cid" => passport.cid = data[4..length].to_string(),
                        _ => println!("{}", data),
                    }
                }
            }
        }
        valid_passports.push(passport);
    }

    return valid_passports;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_birth_low_bounds() {
        let mut passport = Passport::new();
        passport.birth = "1920".to_string();
        assert!(passport.birth_valid());
    }

    #[test]
    fn valid_birth_upper_bounds() {
        let mut passport = Passport::new();
        passport.birth = "2002".to_string();
        assert!(passport.birth_valid());
    }

    #[test]
    fn valid_birth(){
        let mut passport = Passport::new();
        passport.birth = "1980".to_string();
        assert!(passport.birth_valid());
    }

    #[test]
    fn invalid_birth_year() {
        let mut passport = Passport::new();
        passport.birth = "2003".to_string();
        assert!(!passport.birth_valid());
    }

    #[test]
    fn invalid_birth_length() {
        let mut passport = Passport::new();
        passport.birth = "200".to_string();
        assert!(!passport.birth_valid());
    }
    #[test]
    fn valid_issue(){
        let mut passport = Passport::new();
        passport.issue = "2012".to_string();
        assert!(passport.issue_valid());
    }


    #[test]
    fn valid_expiration(){
        let mut passport = Passport::new();
        passport.expiration = "2030".to_string();
        assert!(passport.expiration_valid());
    }

    #[test]
    fn valid_height_in(){
        let mut passport = Passport::new();
        passport.height = "60in".to_string();
        assert!(passport.height_valid());
    }

    #[test]
    fn valid_height_cm(){
        let mut passport = Passport::new();
        passport.height = "190cm".to_string();
        assert!(passport.height_valid());
    }

    #[test]
    fn invalid_height_in(){
        let mut passport = Passport::new();
        passport.height = "190in".to_string();
        assert!(!passport.height_valid());
    }

    #[test]
    fn invalid_height_cm(){
        let mut passport = Passport::new();
        passport.height = "130cm".to_string();
        assert!(!passport.height_valid());
    }

    #[test]
    fn invalid_height(){
        let mut passport = Passport::new();
        passport.height = "190".to_string();
        assert!(!passport.height_valid());
    }

    #[test]
    fn valid_hair_color(){
        let mut passport = Passport::new();
        passport.hair_color = "#ffffff".to_string();
        assert!(passport.hair_valid());
    }

    #[test]
    fn invalid_hair_color(){
        let mut passport = Passport::new();
        passport.hair_color = "#123abg".to_string();
        assert!(!passport.hair_valid());
    }

    #[test]
    fn invalid_hair_color_no_hash(){
        let mut passport = Passport::new();
        passport.hair_color = "123abc".to_string();
        assert!(!passport.hair_valid());
    }

    #[test]
    fn valid_eye_color(){
        let mut passport = Passport::new();
        passport.eye_color = "brn".to_string();
        assert!(passport.eye_valid());
    }

    #[test]
    fn invalid_eye_color(){
        let mut passport = Passport::new();
        passport.eye_color = "wat".to_string();
        assert!(!passport.eye_valid());
    }

    #[test]
    fn valid_pid(){
        let mut passport = Passport::new();
        passport.pid = "000000001".to_string();
        assert!(passport.pid_valid());
    }

    #[test]
    fn invalid_pid(){
        let mut passport = Passport::new();
        passport.pid = "123456789".to_string();
        assert!(!passport.pid_valid());
    }

    #[test]
    fn passport_validity_test(){
        let mut counter = 0;
        let filename = "../adventofcode2020/files/day4-valid.txt";
    
        let valid_passports: Vec<Passport> = collect_passport_information(filename);
        
        println!("number of passports: {}", valid_passports.len());
        println!("{:?}", valid_passports[0]);

        for passport in valid_passports{
            if passport.valid() {
                counter = counter + 1;
            }
        }

        assert_eq!(counter, 4);
    }

    #[test]
    fn passport_invalidity_test(){
        let mut counter = 0;
        let filename = "../adventofcode2020/files/day4-invalid.txt";
    
        let valid_passports: Vec<Passport> = collect_passport_information(filename);

        for passport in valid_passports{
            if passport.valid() {
                counter = counter + 1;
            }
        }

        assert_eq!(counter, 0);
    }
}