use std::io::{self, Write};
use super::types::{ Numbers, Sum };


pub fn write_output(numbers: &Numbers, sum: Sum) {
    let coerced_sum: String = sum.to_string();
    let converted_numbers: Numbers = numbers.to_vec();
    let formatted_line_one: String = format!("numbers: {:?}", converted_numbers);
    let formatted_line_two: String = format!("sum: {}", coerced_sum);
    write_line_to_stdout(formatted_line_one);
    write_line_to_stdout(formatted_line_two);
}

fn write_line_to_stdout(line: String) {
    let mut stdout = io::stdout();
    match writeln!(&mut stdout, "{}", line) {
        Ok(_) => {}
        Err(_error) => panic!("Could not write to stdout!")
    }
}
