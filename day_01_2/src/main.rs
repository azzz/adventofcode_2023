use std::fs::read_to_string;

fn main() {
    let sum: i32 = read_to_string("input.txt").
        unwrap().
        lines().
        fold(0, |acc, line| acc + extract(line));

    println!("total sum: {}", sum)
}

fn extract(input: &str) -> i32 {
    let mut nums: Vec<i32> = Vec::new();

    for (i,_) in input.char_indices() {
        let substr = &input[i..];

        if let Some(d) = maybe_digit(input.chars().nth(i).unwrap()) {
            nums.push(d);
            continue;
        }

        if substr.starts_with("one") {
            nums.push(1);
        } else if substr.starts_with("two") {
            nums.push(2);
        } else if substr.starts_with("three") {
            nums.push(3)
        } else if substr.starts_with("four") {
            nums.push(4)
        } else if substr.starts_with("five") {
            nums.push(5)
        } else if substr.starts_with("six") {
            nums.push(6)
        } else if substr.starts_with("seven") {
            nums.push(7)
        } else if substr.starts_with("eight") {
            nums.push(8)
        } else if substr.starts_with("nine") {
            nums.push(9)
        }
    }

    let mut iter = nums.iter();

    let first = match iter.next() {
        None => 0,
        Some(v) => v.to_string().parse::<i32>().unwrap(),
    };

    match iter.last() {
        None => first * 11,
        Some(v) => first * 10 + v.to_string().parse::<i32>().unwrap(),
    }
}

fn maybe_digit(ch: char) -> Option<i32> {
    match ch.to_string().parse::<i32>() {
        Err(_) => None,
        Ok(v) => Some(v)
    }
}

#[cfg(test)]
mod test {
    use super::extract;

    #[test]
    fn extract_test() {
        assert_eq!(29, extract("two1nine"));
        assert_eq!(83, extract("eightwothree"));
        assert_eq!(13, extract("abcone2threexyz"));
        assert_eq!(24, extract("xtwone3four"));
        assert_eq!(42, extract("4nineeightseven2"));
        assert_eq!(14, extract("zoneight234"));
        assert_eq!(76, extract("7pqrstsixteen"));
    }
}