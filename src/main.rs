mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod utils;

fn main() {
    let input_file = utils::read_file_to_string("inputs/input_06.txt");
    let groups = day_06::read_answers_from_string(&input_file);
    let mut sum = 0;
    for group in groups {
        sum += group.get_same_answers_count();
    }
    println!("{}", sum);
}
