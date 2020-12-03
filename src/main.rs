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

fn day_01_part1(input_path: &'static str) -> Result<u32, &'static str> {
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

fn day_01_part2(input_path: &'static str) -> Result<u32, &'static str> {
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

fn day_02_part1(password_entries: &Vec<PasswordEntry>) -> u32 {
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

fn day_02_part2(password_entries: &Vec<PasswordEntry>) -> u32 {
    // now we can just validate each password
    let mut valid_password_count: u32 = 0;
    for password_entry in password_entries {
        let mut char_entry_count = 0u32;
        if password_entry.min_occurence <= password_entry.password.len()
        {
            if(password_entry.password.chars().nth(password_entry.min_occurence - 1)).unwrap() == password_entry.character
            {
                char_entry_count += 1;
            }
        }

        if password_entry.max_occurence <= password_entry.password.len()
        {
            if(password_entry.password.chars().nth(password_entry.max_occurence - 1)).unwrap() == password_entry.character
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

fn main() {
    let password_entries = parse_password_entries("inputs/input_02.txt");
    println!("Day 2 Part 2 result {}", day_02_part2(&password_entries));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_01_part1_test() {
        assert_eq!(day_01_part1("inputs/input_01.txt"), Ok(567171));
    }

    #[test]
    fn day_01_part2_test() {
        assert_eq!(day_01_part2("inputs/input_01.txt"), Ok(212428694));
    }

    #[test]
    fn day_02_part1_test() {
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
        assert_eq!(day_02_part1(&password_entries), 2);

        let password_entries = parse_password_entries("inputs/input_02.txt");
        assert_eq!(day_02_part1(&password_entries), 410);
    }

    #[test]
    fn day_02_part2_test() {
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
        assert_eq!(day_02_part2(&password_entries), 1);

        let password_entries = parse_password_entries("inputs/input_02.txt");
        assert_eq!(day_02_part2(&password_entries), 694);
    }
}
