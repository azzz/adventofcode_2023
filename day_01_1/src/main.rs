use std::fs::read_to_string;

fn main() {
    let sum: i32 = read_to_string("input.txt").
        unwrap().
        lines().
        fold(0, |acc, line| acc + extract(line));

    println!("total sum: {}", sum)
}

fn extract(input: &str) -> i32 {
    let mut digits = input.chars().filter(|ch| ch.is_digit(10));

    let first = match digits.next() {
        None => 0,
        Some(v) => v.to_string().parse::<i32>().unwrap(),
    };

    match digits.last() {
        None => first * 11,
        Some(v) => first * 10 + v.to_string().parse::<i32>().unwrap(),
    }
}

#[cfg(test)]
mod test {
    use super::extract;

    #[test]
    fn extract_test() {
        assert_eq!(12, extract("1abc2"));
        assert_eq!(38, extract("pqr3stu8vwx"));
        assert_eq!(15, extract("a1b2c3d4e5f"));
        assert_eq!(77, extract("treb7uchet"));
        assert_eq!(33, extract("333"));
        assert_eq!(0, extract("foo"));
    }
}