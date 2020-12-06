use std::fs;

use crate::utils;

#[derive(Debug, PartialEq)]
pub struct Group(Vec<String>);

pub fn read_answers_from_string(input: &str) -> Vec<Group> {
    if input.is_empty() {
        return Vec::new();
    }

    // first split by new lines
    let mut groups: Vec<Group> = Vec::new();
    for group in input.split("\n\n") {
        println!("Group {:#?}", group);
        if group.is_empty() {
            continue;
        }
        // let's split by person
        let mut persons: Vec<String> = Vec::new();
        for person in group.split('\n') {
            println!("Person {:#?}", person);
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
    }
    #[test]
    fn part_01() {}
}
