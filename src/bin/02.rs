use regex::Regex;

#[derive(Debug)]
struct Validator {
    min: u8,
    max: u8,
    count: u8,
}

impl Validator {
    pub fn from_string(string: &str) -> Result<Self, String> {
        let re = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();

        if let Some(cap) = re.captures(string) {
            let groups = (cap.get(1), cap.get(2), cap.get(3), cap.get(4));
            match groups {
                (Some(min), Some(max), Some(c), Some(pwd)) => {
                    let c = c.as_str().chars().next().unwrap();
                    let min = str::parse::<u8>(min.as_str()).unwrap();
                    let max = str::parse::<u8>(max.as_str()).unwrap();
                    let pwd = String::from(pwd.as_str());
                    let count: u8 = pwd.matches(c).count() as u8;
                    return Ok(Validator {
                        min,
                        max,
                        count,
                    });
                }
                _ => {
                    return Err(String::from(""));
                }
            };
        }

        Err(String::from("Could not parse"))
    }

    pub fn is_valid(&self) -> bool {
        (self.count >= self.min) && (self.count <= self.max)
    }
}

fn is_valid_2(line: &str) -> Result<bool, String> {

    let re = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();

        if let Some(cap) = re.captures(line) {
            let groups = (cap.get(1), cap.get(2), cap.get(3), cap.get(4));
            match groups {
                (Some(v1), Some(v2), Some(c), Some(pwd)) => {
                    let c = c.as_str().chars().next().unwrap();
                    let v1 = str::parse::<usize>(v1.as_str()).unwrap();
                    let v2 = str::parse::<usize>(v2.as_str()).unwrap();
                    let pwd = String::from(pwd.as_str()).chars().collect::<Vec<char>>();

                    return Ok((pwd[v1-1]==c) ^ (pwd[v2-1] == c))

                }
                _ => {
                    return Err(String::from("trouble extracting matches"));
                }
            };
        }

    Err(String::from("trouble parsing"))
}

fn main() {
    let input = include_str!("../../inputs/02.txt");

    // Part 1
    let count = input
        .lines()
        .map(|s| Validator::from_string(s).unwrap())
        .filter(|v| v.is_valid())
        .count();

    println!("1) {}", count);

    // Part 2
    let count2 = input
        .lines()
        .filter(|l| is_valid_2(l).unwrap())
        .count();

        println!("2) {}", count2);
}