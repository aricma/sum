use super::types::{ Lines, Numbers };
use std::num::ParseIntError;


pub fn convert_lines_into_numbers(lines: &Lines) -> Numbers {
    let mut numbers: Numbers = Vec::new();
    for line in lines {
        let numbers_from_lines: Numbers = convert_line_to_numbers(line);
        numbers.extend(&numbers_from_lines)
    }
    return numbers;
}

fn convert_line_to_numbers(line: &String) -> Numbers {
    let mut numbers: Numbers = Vec::new();
    let line_split_by_spaces: Vec<&str> = line.split(' ').collect();
    for character in line_split_by_spaces {
        let coerced_character: String = String::from(character);
        let number: Option<i32> = convert_character_to_number(&coerced_character);
        match number {
            Some(number) => numbers.push(number),
            _ => {}
        }
    }
    return numbers;
}

fn convert_character_to_number(character: &String) -> Option<i32> {
    let parsed_character: Result<i32, ParseIntError> = character.parse::<i32>();
    match parsed_character {
        Ok(number) => { return Some(number); }
        Err(_err) => {}
    }
    return None;
}