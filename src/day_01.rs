use std::{fmt::Debug, fs, str::FromStr};

fn parse_list_from_file<T: FromStr + Debug>(input_path: &'static str, separator: char) -> Vec<T> {
    // first open file
    let file_content = fs::read_to_string(input_path).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", input_path);
        "".to_string()
    });
    // now lets convert every lines to a vec of numbers
    let mut element_list: Vec<T> = Vec::new();
    for line in file_content.split(separator) {
        match line.trim().parse::<T>() {
            Ok(element) => element_list.push(element),
            Err(_) => eprintln!("Failed to parse line {}", line),
        }
    }
    element_list
}

pub fn day_01_part_01(input_path: &'static str) -> Result<u32, &'static str> {
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
    Err("Failed to find number")
}

pub fn day_01_part_02(input_path: &'static str) -> Result<u32, &'static str> {
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
    Err("Failed to find number")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_01() {
        assert_eq!(day_01_part_01("inputs/input_01.txt"), Ok(567171));
    }

    #[test]
    fn part_02() {
        assert_eq!(day_01_part_02("inputs/input_01.txt"), Ok(212428694));
    }
}
