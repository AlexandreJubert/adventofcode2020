use std::fs;

#[derive(Debug, std::cmp::PartialEq)]
enum MapElement {
    Empty,
    Tree,
}
struct Slope(usize, usize);

fn parse_tree_map(input_path: &'static str) -> Vec<Vec<MapElement>> {
    let file_content = fs::read_to_string(input_path).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", input_path);
        "".to_string()
    });

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

fn day_03(map: &[Vec<MapElement>], slope: &Slope) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_01() {
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
    fn part_02() {
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
}
