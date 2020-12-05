use std::fs;

#[derive(Debug)]
pub enum Position {
    Front,
    Back,
    Right,
    Left,
}

#[derive(Debug)]
pub struct SeatTicket(pub Vec<Position>);

pub const PLANE_ROW_COUNT: usize = 128;
pub const PLANE_COLUMN_COUNT: usize = 8;

impl SeatTicket {
    pub fn get_seat_position(&self) -> (usize, usize) {
        let mut current_row_range = (0, PLANE_ROW_COUNT - 1);
        let mut current_col_range = (0, PLANE_COLUMN_COUNT - 1);
        for position in &self.0 {
            match position {
                Position::Front => {
                    current_row_range.1 -= (current_row_range.1 - current_row_range.0) / 2 + 1
                }
                Position::Back => {
                    current_row_range.0 += (current_row_range.1 - current_row_range.0) / 2 + 1
                }
                Position::Right => {
                    current_col_range.0 += (current_col_range.1 - current_col_range.0) / 2 + 1
                }
                Position::Left => {
                    current_col_range.1 -= (current_col_range.1 - current_col_range.0) / 2 + 1
                }
            }
        }
        (current_row_range.0, current_col_range.0)
    }

    pub fn get_seat_id(&self) -> usize {
        let current_position = self.get_seat_position();
        current_position.0 * 8 + current_position.1
    }
}

pub fn parse_seats_from_file(input_path: &'static str) -> Vec<SeatTicket> {
    let file_content = fs::read_to_string(input_path).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", input_path);
        "".to_string()
    });

    file_content
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            SeatTicket(
                line.trim()
                    .chars()
                    .map(|character| match character {
                        'F' => Position::Front,
                        'B' => Position::Back,
                        'R' => Position::Right,
                        'L' => Position::Left,
                        _ => panic!("Invalid character"),
                    })
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::cmp;

    use super::*;
    #[test]
    fn part_01() {
        let seat = SeatTicket(vec![
            Position::Front,
            Position::Back,
            Position::Front,
            Position::Back,
            Position::Back,
            Position::Front,
            Position::Front,
            Position::Right,
            Position::Left,
            Position::Right,
        ]);
        assert_eq!(seat.get_seat_position(), (44, 5));
        assert_eq!(seat.get_seat_id(), 357);

        let seats = parse_seats_from_file("inputs/input_05.txt");
        let mut highest_seat = 0usize;
        for seat in seats {
            highest_seat = cmp::max(highest_seat, seat.get_seat_id());
        }

        assert_eq!(highest_seat, 890);
    }
}
