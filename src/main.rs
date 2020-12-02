use std::{env, fmt::Debug, fs, str::FromStr};

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

fn day_01_part1(input_path: &'static str) {
    let number_list: Vec<u32> = parse_list_from_file(input_path, '\n');
    for (number1_index, number1) in number_list.iter().enumerate() {
        for (number2_index, number2) in number_list.iter().enumerate() {
            // lets avoid testing the same indices
            if number1_index == number2_index {
                continue;
            }
            if number1 + number2 == 2020 {
                println!("Day 01 part 1 result: {}", number1 * number2);
                return;
            }
        }
    }
}

fn day_01_part2(input_path: &'static str) {
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
                    return;
                }
            }
        }
    }
}

fn main() {
    // take first argument to path
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return;
    }

    let input_file_path: &String = &args[1];

    println!("Opening file {}", input_file_path);

    day_01_part1("input_01.txt");
    day_01_part2("input_01.txt");
}
