use std::fs;

#[derive(Debug, Clone)]
enum Height {
    Inch(u32),
    Cm(u32),
}

#[derive(Debug)]
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

#[derive(Debug)]
pub struct PassportEntry {
    byr: Option<u32>,      // (Birth Year)
    iyr: Option<u32>,      // (Issue Year)
    eyr: Option<u32>,      // (Expiration Year)
    hgt: Option<Height>,   // (Height)
    hcl: Option<String>,   // (Hair Color)
    ecl: Option<EyeColor>, // (Eye Color)
    pid: Option<String>,   // (Passport ID)
    cid: Option<String>,   // (Country ID)
}

impl PassportEntry {
    pub fn new() -> PassportEntry {
        PassportEntry {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    pub fn is_valid(&self, validate_data: bool) -> bool {
        let mut is_valid = true;
        is_valid &= match self.byr {
            Some(byr) => {
                if validate_data {
                    byr >= 1920 && byr <= 2002
                } else {
                    true
                }
            }
            None => false,
        };

        is_valid &= match self.iyr {
            Some(iyr) => {
                if validate_data {
                    iyr >= 2010 && iyr <= 2020
                } else {
                    true
                }
            }
            None => false,
        };

        is_valid &= match self.eyr {
            Some(eyr) => {
                if validate_data {
                    eyr >= 2020 && eyr <= 2030
                } else {
                    true
                }
            }
            None => false,
        };

        is_valid &= match &self.hgt {
            Some(hgt) => match hgt {
                Height::Inch(value) => {
                    if validate_data {
                        *value >= 59 && *value <= 76
                    } else {
                        true
                    }
                }
                Height::Cm(value) => {
                    if validate_data {
                        *value >= 150 && *value <= 193
                    } else {
                        true
                    }
                }
            },
            None => false,
        };

        is_valid &= match &self.hcl {
            Some(hcl) => {
                if validate_data {
                    hcl[1..].len() == 6
                } else {
                    true
                }
            }
            None => false,
        };
        is_valid &= self.ecl.is_some();
        is_valid &= match &self.pid {
            Some(pid) => {
                if validate_data {
                    pid.chars().all(char::is_numeric) && pid.len() == 9
                } else {
                    true
                }
            }
            None => false,
        };
        is_valid
    }
}

pub fn parse_passport_list(input_path: &'static str) -> Vec<PassportEntry> {
    let file_content = fs::read_to_string(input_path).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", input_path);
        "".to_string()
    });

    let mut passports: Vec<PassportEntry> = Vec::new();

    // split by new line
    for text_passport_entry in file_content.split("\r\n\r\n") {
        let mut passport_entry = PassportEntry::new();
        for passport_line_entry in text_passport_entry.split('\n') {
            for data_entry in passport_line_entry.split(' ') {
                // and finally we split to a key/value pair
                let key_value: Vec<&str> = data_entry.split(':').collect();
                if key_value.len() != 2 {
                    continue; // we skip entries that don't have the right format
                }

                let key = key_value[0];
                let value = key_value[1].trim();

                match key {
                    "byr" => passport_entry.byr = Some(value.parse::<u32>().unwrap()),
                    "iyr" => passport_entry.iyr = Some(value.parse::<u32>().unwrap()),
                    "eyr" => passport_entry.eyr = Some(value.parse::<u32>().unwrap()),
                    "hgt" => {
                        passport_entry.hgt = match &value[value.len() - 2..value.len()] {
                            "cm" => Some(Height::Cm(
                                value[0..value.len() - 2].parse::<u32>().unwrap(),
                            )),
                            "in" => Some(Height::Inch(
                                value[0..value.len() - 2].parse::<u32>().unwrap(),
                            )),
                            _ => None,
                        }
                    }
                    "hcl" => passport_entry.hcl = Some(value.to_string()),
                    "ecl" => {
                        passport_entry.ecl = match value {
                            "amb" => Some(EyeColor::Amb),
                            "blu" => Some(EyeColor::Blu),
                            "brn" => Some(EyeColor::Brn),
                            "gry" => Some(EyeColor::Gry),
                            "grn" => Some(EyeColor::Grn),
                            "hzl" => Some(EyeColor::Hzl),
                            "oth" => Some(EyeColor::Oth),
                            _ => None,
                        }
                    }
                    "pid" => passport_entry.pid = Some(value.to_string()),
                    "cid" => passport_entry.cid = Some(value.to_string()),
                    _ => {}
                }
            }
        }
        passports.push(passport_entry);
    }
    passports
}

pub fn day_04(passport_list: &[PassportEntry], validate_data: bool) -> u32 {
    let mut valid_passport = 0u32;
    for passport in passport_list {
        if passport.is_valid(validate_data) {
            valid_passport += 1;
        }
    }
    valid_passport
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_01() {
        let passports = parse_passport_list("inputs/input_04_example.txt");
        assert_eq!(day_04(&passports, false), 2);

        let passports = parse_passport_list("inputs/input_04.txt");
        assert_eq!(day_04(&passports, false), 213);
    }

    #[test]
    fn part_02() {
        let passports = parse_passport_list("inputs/input_04.txt");
        assert_eq!(day_04(&passports, true), 184);
    }
}
