use std::{collections::HashSet, fs::read_to_string};
use nom::{
    bytes::complete::tag, 
    IResult, combinator::map_res, 
    multi::separated_list1, 
    character::complete::{multispace1, digit1}, 
    sequence::{tuple, separated_pair}
};

fn main() {
    solve_part_1()
}


fn solve_part_1() {
    let sum: u32 = read_to_string("input.txt").
        unwrap().
        lines().
        map(|line| parse(line).unwrap() ).
        map(|c| c.points() ).sum();

    println!("PART 1: {}", sum)
}

struct Card {
    nums: HashSet<u32>,
    wins: HashSet<u32>,
    copies: u32,
}

impl Card {
    fn points(&self) -> u32 {
        let n = self.win_nums();

        if n == 0 { return 0 }
        (2 as u32).pow(n-1)
    }

    fn win_nums(&self) -> u32 {
        self.copies * self.nums.intersection(&self.wins).count() as u32
    }
}

impl From<(Vec<u32>, Vec<u32>)> for Card {
    fn from((nums, wins): (Vec<u32>, Vec<u32>)) -> Self {
        let mut nums_set = HashSet::new();
        let mut wins_set = HashSet::new();

        for n in nums.iter() { nums_set.insert(*n); };
        for n in wins.iter() { wins_set.insert(*n); };

        Self { nums: nums_set, wins: wins_set, copies: 1 }
    }
}

fn parse(input: &str) -> Option<Card> {
    let nums = match parse_card(input) {
        Err(_) => return None,
        Ok((_, nums)) => nums,
    };

    Some(nums.into())
}

fn parse_card(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let nums_parser = |input| separated_list1(multispace1, map_res(digit1, |x: &str| x.parse::<u32>()))(input);
    
    // throw away "Card 1: "
    let (input, _) = tuple((tag("Card"), multispace1, digit1, tag(":"), multispace1))(input)?;
    separated_pair(
        nums_parser,
        tuple((multispace1, tag("|"), multispace1)),
        nums_parser,
    )(input)
}

#[cfg(test)]
mod test {
    use crate::parse_card;

    #[test]
    fn parse_card_test() {
        assert_eq!(parse_card("Card 1: 7  43 12 |  33 12 5"), Ok(("", (vec![7, 43, 12], vec![33, 12, 5]))));
    }
}

#[cfg(test)]
mod card_test {
    use crate::Card;

    #[test]
    fn points() {
        {
            let card = Card([41,48,83,86,17].into(), [83, 86, 6, 31, 17, 9, 48, 53].into());
            assert_eq!(card.points(), 8);
        }

        {
            let card = Card([87, 83, 26, 28, 32].into(), [88, 30, 70, 12, 93, 22, 82, 36].into());
            assert_eq!(card.points(), 0);
        }
    }
}