use std::{fmt::Debug, fs, str::FromStr};

fn parse_list_from_file<T: FromStr + Debug>(input_path: &'static str, separator: char) -> Vec<T> {
    // first open file
    let file_content = fs::read_to_string(input_path)
        .expect(format!("Failed to read file {}", input_path).as_str());
    // now lets convert every lines to a vec of numbers
    let mut element_list: Vec<T> = Vec::new();
    for line in file_content.split(separator) {
        match line.parse::<T>() {
            Ok(element) => element_list.push(element),
            Err(_) => eprintln!("Failed to parse line {}", line),
        }
    }
    element_list
}

fn day_01_part_01(input_path: &'static str) -> Result<u32, &'static str> {
    let number_list: Vec<u32> = parse_list_from_file(input_path, '\n');
    for (number1_index, number1) in number_list.iter().enumerate() {
        for (number2_index, number2) in number_list.iter().enumerate() {
            // lets avoid testing the same indices
            if number1_index == number2_index {
                continue;
            }
            if number1 + number2 == 2020 {
                println!("Day 01 part 1 result: {}", number1 * number2);
                return Ok(number1 * number2);
            }
        }
    }
    return Err("Failed to find number");
}

fn day_01_part_02(input_path: &'static str) -> Result<u32, &'static str> {
    let number_list: Vec<u32> = parse_list_from_file(input_path, '\n');

    // now we'll iterate the list twice
    for (number1_index, number1) in number_list.iter().enumerate() {
        for (number2_index, number2) in number_list.iter().enumerate() {
            // lets avoid testing the same indices
            if number1_index == number2_index {
                continue;
            }

            for (number3_index, number3) in number_list.iter().enumerate() {
                // lets avoid testing the same indices
                if number1_index == number3_index || number2_index == number3_index {
                    continue;
                }

                if number1 + number2 + number3 == 2020 {
                    println!("Day 01 part 2 result: {}", number1 * number2 * number3);
                    return Ok(number1 * number2 * number3);
                }
            }
        }
    }
    return Err("Failed to find number");
}

#[derive(Debug)]
struct PasswordEntry {
    min_occurence: usize,
    max_occurence: usize,
    character: char,
    password: String,
}

fn parse_password_entries(input_path: &'static str) -> Vec<PasswordEntry> {
    let file_content = fs::read_to_string(input_path)
        .expect(format!("Failed to read file {}", input_path).as_str());
    // now we need to split and parse a bunch of stuff
    // we'll go line by line
    let mut password_entries: Vec<PasswordEntry> = Vec::new();
    for line in file_content.split('\n') {
        if line.is_empty() {
            continue;
        }
        // now we split by space our line, we collect a list of str references to the line content
        let elements: Vec<&str> = line.split(' ').collect();
        if elements.len() != 3 {
            eprintln!("Failed to parse line {}, invalid format.", line);
            continue;
        }

        // now we parse the entry
        // first the min max range
        let range: Vec<&str> = elements[0].split('-').collect();
        if range.len() != 2 {
            eprintln!("Failed to parse value range {:?} at line {}.", range, line);
            continue;
        }

        let min_occurence: usize = range[0].parse().unwrap();
        let max_occurence: usize = range[1].parse().unwrap();

        // next we parse the character
        let character = elements[1].chars().nth(0).unwrap();

        // then the password itself we can just copy it into a string
        let password = elements[2].to_string();

        password_entries.push(PasswordEntry {
            min_occurence,
            max_occurence,
            character,
            password,
        });
    }
    password_entries
}

fn day_02_part_01(password_entries: &Vec<PasswordEntry>) -> u32 {
    // now we can just validate each password
    let mut valid_password_count: u32 = 0;
    for password in password_entries {
        // now we can just validate, we need to count the character occurence in the string
        let character_count = password.password.matches(password.character).count();
        if character_count >= password.min_occurence && character_count <= password.max_occurence {
            valid_password_count += 1;
        }
    }
    valid_password_count
}

fn day_02_part_02(password_entries: &Vec<PasswordEntry>) -> u32 {
    // now we can just validate each password
    let mut valid_password_count: u32 = 0;
    for password_entry in password_entries {
        let mut char_entry_count = 0u32;
        if password_entry.min_occurence <= password_entry.password.len() {
            if (password_entry
                .password
                .chars()
                .nth(password_entry.min_occurence - 1))
            .unwrap()
                == password_entry.character
            {
                char_entry_count += 1;
            }
        }

        if password_entry.max_occurence <= password_entry.password.len() {
            if (password_entry
                .password
                .chars()
                .nth(password_entry.max_occurence - 1))
            .unwrap()
                == password_entry.character
            {
                char_entry_count += 1;
            }
        }

        if char_entry_count == 1 {
            valid_password_count += 1;
        }
    }
    valid_password_count
}

#[derive(Debug, std::cmp::PartialEq)]
enum MapElement {
    Empty,
    Tree,
}
struct Slope(usize, usize);

fn parse_tree_map(input_path: &'static str) -> Vec<Vec<MapElement>> {
    let file_content = fs::read_to_string(input_path)
        .expect(format!("Failed to read file {}", input_path).as_str());

    let mut map: Vec<Vec<MapElement>> = Vec::new();
    for line in file_content.split('\n') {
        if line.is_empty() {
            continue;
        } // skip empty lines
        let mut current_row: Vec<MapElement> = Vec::new();
        for character in line.chars() {
            match character {
                '.' => current_row.push(MapElement::Empty),
                '#' => current_row.push(MapElement::Tree),
                _ => {}
            }
        }
        map.push(current_row);
    }
    map
}

fn day_03(map: &Vec<Vec<MapElement>>, slope: &Slope) -> u32 {
    // now we can iterate in our map
    // first lets get the map dimensions
    if map.is_empty() || map[0].is_empty() {
        return 0;
    }

    let width = map[0].len();
    let mut current_column: usize = 0;
    let mut total_tree_count = 0;
    for current_row in map.iter().step_by(slope.1 as usize) {
        if current_row[current_column % width] == MapElement::Tree {
            total_tree_count += 1;
        }
        current_column += slope.0; // we increment according to our slope parameter, which is one down 3 right
    }
    total_tree_count
}

#[derive(Debug)]

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
struct PassportEntry {
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
    fn new() -> PassportEntry {
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

    fn is_valid(&self) -> bool {
        let mut is_valid = true;
        is_valid &= match self.byr {
            Some(byr) => byr >= 1920 && byr <= 2002,
            None => false,
        };

        is_valid &= match self.iyr {
            Some(iyr) => iyr >= 2010 && iyr <= 2020,
            None => false,
        };

        is_valid &= match self.eyr {
            Some(eyr) => eyr >= 2020 && eyr <= 2030,
            None => false,
        };

        is_valid &= match self.hgt {
            Some(hgt) => match hgt {
                Height::Inch(value) => value >= 59 && value <= 76,
                Height::Cm(value) => value >= 150 && value <= 193,
            },
            None => false,
        };

        is_valid &= self.hcl.is_some();
        is_valid &= self.ecl.is_some();
        is_valid &= self.pid.is_some();

        is_valid
    }
}

fn parse_passport_list(input_path: &'static str) -> Vec<PassportEntry> {
    let file_content = fs::read_to_string(input_path)
        .expect(format!("Failed to read file {}", input_path).as_str());

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
                match key_value[0] {
                    "byr" => passport_entry.byr = Some(key_value[1].parse::<u32>().unwrap()),
                    "iyr" => passport_entry.iyr = Some(key_value[1].parse::<u32>().unwrap()),
                    "eyr" => passport_entry.eyr = Some(key_value[1].parse::<u32>().unwrap()),
                    "hgt" => {
                        passport_entry.hgt = {
                            match &key_value[1][key_value.len() - 2..key_value.len()] {
                                "cm" => Some(Height::Cm(
                                    key_value[0..key_value.len() - 2].parse::<u32>().unwrap(),
                                )),
                                "in" => None,
                                _ => None,
                            }
                        }
                    }
                    "hcl" => passport_entry.hcl = Some(key_value[1].to_string()),
                    //"ecl" => passport_entry.ecl = Some(key_value[1].to_string()),
                    "pid" => passport_entry.pid = Some(key_value[1].to_string()),
                    "cid" => passport_entry.cid = Some(key_value[1].to_string()),
                    _ => {}
                }
            }
        }
        passports.push(passport_entry);
    }
    passports
}

fn day_04(passport_list: &Vec<PassportEntry>) -> u32 {
    let mut valid_passport = 0u32;
    for passport in passport_list {
        if passport.is_valid() {
            valid_passport += 1;
        }
    }
    valid_passport
}

fn main() {
    let passports = parse_passport_list("inputs/input_04.txt");
    println!("{}", day_04(&passports));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_01_part_01_test() {
        assert_eq!(day_01_part_01("inputs/input_01.txt"), Ok(567171));
    }

    #[test]
    fn day_01_part_02_test() {
        assert_eq!(day_01_part_02("inputs/input_01.txt"), Ok(212428694));
    }

    #[test]
    fn day_02_part_01_test() {
        let password_entries = vec![
            PasswordEntry {
                min_occurence: 1,
                max_occurence: 3,
                character: 'a',
                password: "abcde".to_string(),
            },
            PasswordEntry {
                min_occurence: 1,
                max_occurence: 3,
                character: 'b',
                password: "cdefg".to_string(),
            },
            PasswordEntry {
                min_occurence: 2,
                max_occurence: 9,
                character: 'c',
                password: "ccccccccc".to_string(),
            },
        ];
        assert_eq!(day_02_part_01(&password_entries), 2);

        let password_entries = parse_password_entries("inputs/input_02.txt");
        assert_eq!(day_02_part_01(&password_entries), 410);
    }

    #[test]
    fn day_02_part_02_test() {
        let password_entries = vec![
            PasswordEntry {
                min_occurence: 1,
                max_occurence: 3,
                character: 'a',
                password: "abcde".to_string(),
            },
            PasswordEntry {
                min_occurence: 1,
                max_occurence: 3,
                character: 'b',
                password: "cdefg".to_string(),
            },
            PasswordEntry {
                min_occurence: 2,
                max_occurence: 9,
                character: 'c',
                password: "ccccccccc".to_string(),
            },
        ];
        assert_eq!(day_02_part_02(&password_entries), 1);

        let password_entries = parse_password_entries("inputs/input_02.txt");
        assert_eq!(day_02_part_02(&password_entries), 694);
    }

    #[test]
    fn day_03_part_01_test() {
        {
            let map = parse_tree_map("inputs/input_03_example.txt");
            assert_eq!(day_03(&map, &Slope(3, 1)), 7);
        }
        {
            let map = parse_tree_map("inputs/input_03.txt");
            assert_eq!(day_03(&map, &Slope(3, 1)), 211);
        }
    }

    #[test]
    fn day_03_part_02_test() {
        let slope_list = vec![
            Slope(1, 1),
            Slope(3, 1),
            Slope(5, 1),
            Slope(7, 1),
            Slope(1, 2),
        ];

        {
            let map = parse_tree_map("inputs/input_03_example.txt");
            let mut total_trees = 1;
            for slope in &slope_list {
                total_trees *= day_03(&map, &slope);
            }
            assert_eq!(total_trees, 336);
        }

        {
            let map = parse_tree_map("inputs/input_03.txt");
            let mut total_trees = 1;
            for slope in &slope_list {
                total_trees *= day_03(&map, &slope);
            }
            assert_eq!(total_trees, 3584591857);
        }
    }

    #[test]
    fn day_04_part_01_test() {
        let passports = parse_passport_list("inputs/input_04_example.txt");
        assert_eq!(day_04(&passports), 2);

        let passports = parse_passport_list("inputs/input_04.txt");
        assert_eq!(day_04(&passports), 2);
    }
}
