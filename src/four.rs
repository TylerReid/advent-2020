use std::fs;
use regex::Regex;

pub fn day_four() {
    let file = fs::read_to_string("input/day4.txt").unwrap();
    let groups: Vec<PassportValidation> = file.split("\n\n")
        .map(|x| parse(x))
        .collect();

    let mut num_valid = 0;
    for p in groups.iter() {
        if p.is_valid() {
            num_valid += 1
        }
    }
    println!("{} valid passports", num_valid)
}

fn parse(s: &str) -> PassportValidation {
    //this is dumb but I don't know how to do this in regex or if I could
    let mut byr = BYR.is_match(s);
    if byr {
        let v = BYR.captures(s).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
        byr = 1920 <= v && v <= 2002;
    }

    let mut iyr = IYR.is_match(s);
    if iyr {
        let v = IYR.captures(s).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
        iyr = 2010 <= v && v <= 2020;
    }

    let mut eyr = EYR.is_match(s);
    if eyr {
        let v = EYR.captures(s).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
        eyr = 2020 <= v && v <= 2030;
    }

    let mut hgt = HGT.is_match(s);
    if hgt {
        let v = HGT.captures(s).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
        let u = HGT.captures(s).unwrap().get(2).unwrap().as_str();
        if u == "in" {
            hgt = 59 <= v && v <= 76
        }
        if u == "cm" {
            hgt = 150 <= v && v <= 193
        }
    }

    PassportValidation {
        byr: byr,//done
        iyr: iyr,//done
        eyr: eyr,//done
        hgt: hgt,//done
        hcl: HCL.is_match(s),//done
        ecl: ECL.is_match(s),//done
        pid: PID.is_match(s),//done
        cid: CID.is_match(s),//done
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
    fn is_valid(&self) -> bool {
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
    static ref BYR: Regex = Regex::new("byr:(\\d{4})( |\n|$)").unwrap();
    static ref IYR: Regex = Regex::new("iyr:(\\d{4})( |\n|$)").unwrap();
    static ref EYR: Regex = Regex::new("eyr:(\\d{4})( |\n|$)").unwrap();
    static ref HGT: Regex = Regex::new("hgt:(\\d*)(cm|in)( |\n|$)").unwrap();
    static ref HCL: Regex = Regex::new("hcl:#([0-9a-f]{6})( |\n|$)").unwrap();
    static ref ECL: Regex = Regex::new("ecl:(amb|blu|brn|gry|grn|hzl|oth)( |\n|$)").unwrap();
    static ref PID: Regex = Regex::new("pid:\\d{9}( |\n|$)").unwrap();
    static ref CID: Regex = Regex::new("cid:\\S*").unwrap();
}