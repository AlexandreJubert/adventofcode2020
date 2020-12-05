use std::fs;

#[derive(Debug)]
struct PasswordEntry {
    min_occurence: usize,
    max_occurence: usize,
    character: char,
    password: String,
}

fn parse_password_entries(input_path: &'static str) -> Vec<PasswordEntry> {
    let file_content = fs::read_to_string(input_path).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", input_path);
        "".to_string()
    });
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
        let character = elements[1].chars().next().unwrap();

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

fn day_02_part_01(password_entries: &[PasswordEntry]) -> u32 {
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

fn day_02_part_02(password_entries: &[PasswordEntry]) -> u32 {
    // now we can just validate each password
    let mut valid_password_count: u32 = 0;
    for password_entry in password_entries {
        let mut char_entry_count = 0u32;
        if password_entry.min_occurence <= password_entry.password.len()
            && (password_entry
                .password
                .chars()
                .nth(password_entry.min_occurence - 1))
            .unwrap()
                == password_entry.character
        {
            char_entry_count += 1;
        }

        if password_entry.max_occurence <= password_entry.password.len()
            && (password_entry
                .password
                .chars()
                .nth(password_entry.max_occurence - 1))
            .unwrap()
                == password_entry.character
        {
            char_entry_count += 1;
        }

        if char_entry_count == 1 {
            valid_password_count += 1;
        }
    }
    valid_password_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_01() {
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
    fn part_02() {
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
}
