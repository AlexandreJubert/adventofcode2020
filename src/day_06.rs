use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Group(Vec<String>);

impl Group {
    pub fn get_unique_answers_count(&self) -> usize {
        // need unique list of characters
        let mut unique: HashSet<char> = HashSet::new();
        for person in &self.0 {
            for character in person.chars() {
                unique.insert(character);
            }
        }
        unique.len()
    }

    pub fn get_same_answers_count(&self) -> usize {
        // we need to iterate over every characters
        let matching_string = "abcdefghijklmnopqrstuvwxyz";
        let mut unique: HashSet<char> = HashSet::new();
        for character in matching_string.chars() {
            let mut is_not_in_all_person_answers = false;
            for person in &self.0 {
                is_not_in_all_person_answers |= person.find(character).is_none();
            }
            if !is_not_in_all_person_answers {
                unique.insert(character);
            }
        }
        unique.len()
    }
}

pub fn read_answers_from_string(input: &str) -> Vec<Group> {
    if input.is_empty() {
        return Vec::new();
    }

    // first split by new lines
    let mut groups: Vec<Group> = Vec::new();
    for group in input.split("\n\n") {
        if group.is_empty() {
            continue;
        }
        // let's split by person
        let mut persons: Vec<String> = Vec::new();
        for person in group.split('\n') {
            if person.is_empty() {
                continue;
            }
            persons.push(person.trim().to_string());
        }
        groups.push(Group(persons));
    }
    groups
}
#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn parsing() {
        let test_string = "abcx\nabcy\nabcz".to_string();
        let answer = vec![Group(vec![
            "abcx".to_string(),
            "abcy".to_string(),
            "abcz".to_string(),
        ])];

        assert_eq!(read_answers_from_string(&test_string), answer);
        assert_eq!(answer[0].get_unique_answers_count(), 6);
    }
    #[test]
    fn part_01() {
        let input_file = utils::read_file_to_string("inputs/input_06.txt");
        let groups = read_answers_from_string(&input_file);
        let mut sum = 0;
        for group in groups {
            sum += group.get_unique_answers_count();
        }
        assert_eq!(sum, 6504);
    }

    #[test]
    fn part_02() {
        let input_file = utils::read_file_to_string("inputs/input_06.txt");
        let groups = read_answers_from_string(&input_file);
        let mut sum = 0;
        for group in groups {
            sum += group.get_same_answers_count();
        }
        assert_eq!(sum, 3351);
    }
}
