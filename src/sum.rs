use super::types::{ Numbers, Number, Sum };


pub fn sum(numbers: &Numbers) -> Sum {
    let mut result: Number = 0;
    for number in numbers.into_iter() {
        result += *number;
    }
    return result;
}

#[cfg(test)]
mod test_sum {
    use super::*;

    #[test]
    fn sum_given_numbers() {
        let numbers: Numbers = vec![1, 2, 3];
        let expected: Sum = 6;
        assert_eq!(sum(&numbers), expected);
    }

}