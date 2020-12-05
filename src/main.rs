mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

use std::cmp;

use day_05::{parse_seats_from_file, Position, SeatTicket};

fn main() {
    let seats = parse_seats_from_file("inputs/input_05_example.txt");
    let mut highest_seat = 0usize;
    for seat in seats {
        highest_seat = cmp::max(highest_seat, seat.get_seat_id());
    }

    println!("{}", highest_seat);
}
