use std::{collections::{HashSet, LinkedList}, fs::read_to_string};
use nom::{
    bytes::complete::tag, 
    IResult, combinator::map_res, 
    multi::separated_list1, 
    character::complete::{multispace1, digit1}, 
    sequence::{tuple, separated_pair}
};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("PART 1: {}", solve_part_1(&input));
    println!("PART 2: {}", solve_part_2(&input));
}

fn solve_part_1(input: &String) -> u32 {
    input.lines().
        map(|line| parse(line).unwrap() ).
        map(|c| c.points() ).sum()
}

fn solve_part_2(input: &String) -> u32 {
    let mut sum: u32 = 0;
    let mut extras: LinkedList<u32> = LinkedList::new();

    for line in input.lines() {
        let card = parse(line).unwrap();
        let copies = extras.pop_front().unwrap_or(0) + 1;
        let matches = card.matches();
        let mut updates = LinkedList::new();

        for _ in 0..matches {
            match extras.pop_front() {
                Some(v) => updates.push_back(v + copies),
                None => updates.push_back(copies)
            }
        }

        updates.append(&mut extras);
        extras = updates;

        sum += copies;

    }
    sum
}

struct Card {
    nums: HashSet<u32>,
    wins: HashSet<u32>,
}

impl Card {
    fn points(&self) -> u32 {
        let n = self.matches();

        if n == 0 { return 0 }
        (2 as u32).pow(n-1)
    }

    fn matches(&self) -> u32 {
        self.nums.intersection(&self.wins).count() as u32
    }
}

impl From<(Vec<u32>, Vec<u32>)> for Card {
    fn from((nums, wins): (Vec<u32>, Vec<u32>)) -> Self {
        let mut nums_set = HashSet::new();
        let mut wins_set = HashSet::new();

        for n in nums.iter() { nums_set.insert(*n); };
        for n in wins.iter() { wins_set.insert(*n); };

        Self { nums: nums_set, wins: wins_set }
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
    use crate::{parse_card, solve_part_2};

    #[test]
    fn parse_card_test() {
        assert_eq!(parse_card("Card 1: 7  43 12 |  33 12 5"), Ok(("", (vec![7, 43, 12], vec![33, 12, 5]))));
    }

    #[test]
    fn solve_part_2_test() {
        {
            let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n".to_owned() +
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n" +
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n" +
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n" +
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n" +
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n";


            assert_eq!(solve_part_2(&input), 30)
        }
    }
}

#[cfg(test)]
mod card_test {
    use crate::Card;

    #[test]
    fn points() {
        {
            let card = Card{
                nums: [41,48,83,86,17].into(), 
                wins: [83, 86, 6, 31, 17, 9, 48, 53].into()
            };
            assert_eq!(card.points(), 8);
        }

        {
            let card = Card{
                nums: [87, 83, 26, 28, 32].into(), 
                wins: [88, 30, 70, 12, 93, 22, 82, 36].into()
            };
            assert_eq!(card.points(), 0);
        }
    }
}