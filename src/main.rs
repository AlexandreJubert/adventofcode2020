use std::{ fmt::Debug, fs, str::FromStr};

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
                return Ok(number1 * number2)
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

fn main() {
    println!("Day 1 Part 1 result {}", day_01_part1("input_01.txt").unwrap());
    println!("Day 1 Part 2 result {}", day_01_part2("input_01.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_01_part1_test() {
        assert_eq!(day_01_part1("input_01.txt"), Ok(567171));
    }

    #[test]
    fn day_02_part2_test() {
        assert_eq!(day_01_part2("input_01.txt"), Ok(212428694));
    }
}