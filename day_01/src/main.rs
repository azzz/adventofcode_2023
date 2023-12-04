use std::fs::read_to_string;

use nom::{
    combinator::value,
    bytes::complete::{tag, take}, 
    IResult, 
    branch::alt, 
    multi::many0,
};

fn main() {
    let sum: u32 = read_to_string("input.txt").
        unwrap().
        lines().
        fold(0, |acc, line| acc + parse(line));

    println!("total sum: {}", sum)
}

fn parse_digit(input: &str) -> IResult<&str, Option<u32>> {
    let (tail, result) = alt(
        (
            value(0, tag("0")), value(0, tag("zero")), 
            value(1, tag("1")), value(1, tag("one")),
            value(2, tag("2")), value(2, tag("two")),
            value(3, tag("3")), value(3, tag("three")),
            value(4, tag("4")), value(4, tag("four")),
            value(5, tag("5")), value(5, tag("five")),
            value(6, tag("6")), value(6, tag("six")),
            value(7, tag("7")), value(7, tag("seven")),
            value(8, tag("8")), value(8, tag("eight")),
            value(9, tag("9")), value(9, tag("nine")),

        )
    )(input)?;

    Ok((tail, Some(result)))
}

fn parse_num(input: &str) -> Vec<u32> {
    match many0(
        alt((
            parse_digit,
            value(None, take(1usize)),
        ))
    )(input) {
        Err(_) => vec![],
        Ok((_, result)) => result.into_iter().flatten().collect()
    }
}

fn parse(input: &str) -> u32 {
    let nums = parse_num(input);
    let first = nums.first().unwrap();
    if let Some(last) = nums.last() {
        first * 10 + last
    } else {
        first * 11
    }
}

#[cfg(test)]
mod test {
    use super::{parse_digit, parse_num, parse};

    #[test]
    fn parse_digit_test() {
        assert_eq!(parse_digit("73").unwrap(), ("3", Some(7)));
        assert_eq!(parse_digit("sevenx").unwrap(), ("x", Some(7)));
    }

    #[test]
    fn parse_num_test() {
        assert_eq!(vec![2,1,9], parse_num("two1nine"));
        assert_eq!(vec![8,3], parse_num("eightwothree"));
        assert_eq!(vec![1,2,3], parse_num("abcone2threexyz"));
        assert_eq!(vec![2,3,4], parse_num("xtwone3four"));
        assert_eq!(vec![4,9,8,7,2], parse_num("4nineeightseven2"));
        assert_eq!(vec![1,2,3,4], parse_num("zoneight234"));
        assert_eq!(vec![7,6], parse_num("7pqrstsixteen"));
    }

    #[test]
    fn parse_test() {
        assert_eq!(29, parse("two1nine"));
        assert_eq!(83, parse("eightwothree"));
        assert_eq!(13, parse("abcone2threexyz"));
        assert_eq!(24, parse("xtwone3four"));
        assert_eq!(42, parse("4nineeightseven2"));
        assert_eq!(14, parse("zoneight234"));
        assert_eq!(76, parse("7pqrstsixteen"));
    }
}