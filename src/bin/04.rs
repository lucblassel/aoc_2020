use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../inputs/04.txt");

    let mut records: Vec<Vec<&str>> = vec![];
    let mut record: Vec<&str> = vec![];

    // parse records
    for line in input.lines() {
        if line.is_empty() {
            records.push(record.clone());
            record = vec![]
        } else {
            let mut fields: Vec<&str> = line.split(' ').collect();
            record.append(&mut fields);
        }
    }
    records.push(record.clone());

    let mandatory_fields: HashSet<String> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .map(|v| v.to_owned())
        .collect();

    // validate records:
    let mut valid = 0;
    for record in records {
        if record.len() < 7 {
            continue;
        }

        let mut record_set: HashMap<String, String> = HashMap::new();
        for field in record.iter() {
            let field: Vec<String> = field.split(':').map(|v| v.to_owned()).collect();
            record_set.insert(field[0].clone(), field[1].clone());
        }

        let passport = validation::Passport {
            entries: record_set,
            mandatory: &mandatory_fields,
        };

        if passport.is_valid() {
            valid += 1;
        }
    }

    println!("{valid} valid passports");
}

pub mod validation {
    use regex::Regex;
    use std::{collections::{HashMap, HashSet}};
    use std::fmt;

    #[derive(Debug)]
    pub struct Passport<'a> {
        pub entries: HashMap<String, String>,
        pub mandatory: &'a HashSet<String>,
    }

    impl<'a> fmt::Display for Passport<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
            let mut formated: Vec<String> = vec![];

            for key in keys {
                let val = match self.entries.get(key) {
                    Some(v) => v,
                    None => "?",
                };
                formated.push(format!("{key}: {val}"));
            }

            write!(f, "{{{}}}", formated.join(", "))
        }
    }

    impl<'a> Passport<'a> {
        pub fn is_valid(&'a self) -> bool {
            if !self.has_valid_fields() {
                return false;
            }

            for (key, val) in self.entries.iter() {
                let valid_field = match key.as_str() {
                    "byr" => is_valid_date(val.as_str(), 1920, 2002),
                    "iyr" => is_valid_date(val.as_str(), 2010, 2020),
                    "eyr" => is_valid_date(val.as_str(), 2020, 2030),
                    "ecl" => matches!(
                        val.as_str(),
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                    ),
                    "pid" => matches_regex(val.as_str(), r"^[0-9]{9}$"),
                    "hcl" => matches_regex(val.as_str(), r"^#[0-9a-f]{6}$"),
                    "hgt" => is_valid_height(val.as_str()),
                    "cid" => true,
                    _ => false,
                };

                if !valid_field {
                    return false;
                }
            }
            true
        }

        fn has_valid_fields(&'a self) -> bool {
            let keys: HashSet<String> = self.entries.keys().cloned().collect();
            (self.mandatory - &keys).is_empty()
        }
    }

    fn is_valid_date(date_string: &str, min: i32, max: i32) -> bool {
        if date_string.len() != 4 {
            return false;
        }

        if let Ok(year) = date_string.parse::<i32>() {
            return year <= max && year >= min;
        }

        false
    }

    fn matches_regex(value: &str, regex: &str) -> bool {
        let re = Regex::new(regex).unwrap();
        re.is_match(value)
    }

    fn is_valid_height(height: &str) -> bool {
        let re: Regex = Regex::new(r"([0-9]{2,3})(cm|in)").unwrap();
        let caps = match re.captures(height) {
            Some(c) => c,
            None => return false,
        };

        let height = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        match caps.get(2).unwrap().as_str() {
            "cm" => (150..=193).contains(&height),
            "in" => (59..=76).contains(&height),
            _ => false,
        }
    }
}
