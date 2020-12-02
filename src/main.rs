use std::{env, fs};

fn day_01(input_path : &String)
{
    // first open file
    let file_content = fs::read_to_string(input_path).expect("Failed to open input file");
    // now lets convert every lines to a vec of numbers
    let mut number_list: Vec<u32> = Vec::new();
    
    for line in file_content.split('\n')
    {
        let number : u32 = line.parse::<u32>().unwrap();
        number_list.push(number);
    }

    // now we'll iterate the list twice
    for (number1_index, number1) in number_list.iter().enumerate()
    {
        for (number2_index, number2) in number_list.iter().enumerate()
        {
            if number1_index == number2_index { continue; } // lets avoid testing the same indices
            for (number3_index, number3) in number_list.iter().enumerate()
            {
                if number1_index == number3_index || number2_index == number3_index { continue; } // lets avoid testing the same indices
                if number1 + number2 + number3 == 2020
                {
                    println!("Result {}", number1 * number2 * number3);
                    return;
                }
            }
        }
    }

}


fn main() {
    // take first argument to path
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 { return; }

    let input_file_path : &String = &args[1];

    println!("Opening file {}", input_file_path);

    day_01(input_file_path);    
}
