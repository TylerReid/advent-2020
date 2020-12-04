use std::fs;
use regex::Regex;

pub fn day_four() {
    let file = fs::read_to_string("input/day4.txt").unwrap();
    let groups: Vec<PassportValidation> = file.split("\n\n")
        .map(|x| parse(x))
        .collect();

    let mut num_valid = 0;
    for p in groups.iter() {
        if p.is_valid_skipping_cid() {
            num_valid += 1
        }
    }
    println!("{} valid passports", num_valid)
}

fn parse(s: &str) -> PassportValidation {
    PassportValidation {
        byr: BYR.is_match(s),
        iyr: IYR.is_match(s),
        eyr: EYR.is_match(s),
        hgt: HGT.is_match(s),
        hcl: HCL.is_match(s),
        ecl: ECL.is_match(s),
        pid: PID.is_match(s),
        cid: CID.is_match(s),
    }
}

#[derive(Debug)]
struct PassportValidation {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

impl PassportValidation {
    fn is_valid_skipping_cid(&self) -> bool {
        self.byr &&
        self.iyr &&
        self.eyr &&
        self.hgt &&
        self.hcl &&
        self.ecl &&
        self.pid
    }
}

lazy_static! {
    static ref BYR: Regex = Regex::new("byr:\\S*").unwrap();
    static ref IYR: Regex = Regex::new("iyr:\\S*").unwrap();
    static ref EYR: Regex = Regex::new("eyr:\\S*").unwrap();
    static ref HGT: Regex = Regex::new("hgt:\\S*").unwrap();
    static ref HCL: Regex = Regex::new("hcl:\\S*").unwrap();
    static ref ECL: Regex = Regex::new("ecl:\\S*").unwrap();
    static ref PID: Regex = Regex::new("pid:\\S*").unwrap();
    static ref CID: Regex = Regex::new("cid:\\S*").unwrap();
}