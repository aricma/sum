mod types;
mod read;
mod convert;
mod sum;
mod write;

use types::{ Lines, Numbers, Sum };
use read::read_input;
use convert::convert_lines_into_numbers;
use sum::sum;
use write::write_output;


pub fn main() {
    let ref lines: Lines = read_input();
    let ref numbers: Numbers = convert_lines_into_numbers(lines);
    let sum: Sum = sum(numbers);
    write_output(numbers, sum);
}
