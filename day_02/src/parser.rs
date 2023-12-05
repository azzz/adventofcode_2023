use crate::game::*;

use nom::{
    IResult, 
    sequence::{separated_pair, delimited}, 
    bytes::complete::tag, 
    multi::separated_list1, 
    character::complete::{digit1, alpha1}, 
    combinator::map_res
};

fn parse_game_id(input: &str) -> IResult<&str, u32> {
    map_res(
        delimited(tag("Game "), digit1, tag(":")),
        |x: &str| x.parse::<u32>()
    )(input)
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let (input, cube) = separated_pair(
        map_res(digit1, |x: &str| x.parse::<u32>()), 
        tag(" "), 
        alpha1
    )(input)?;

    Ok((input, cube.into()))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    separated_list1(tag(", "), parse_cube)(input)
}

pub fn parse_game(input: &str) -> IResult<&str, Game> {
    let parse_hands = separated_list1(tag("; "), parse_hand);
    let (input, (id, hands)) = separated_pair(parse_game_id, tag(" "), parse_hands)(input)?;

    Ok((input, Game::new(id, hands)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_game_id_test() {
        assert_eq!(parse_game_id("Game 7:"), 
            Ok(("", 7))
        );
        assert_eq!(parse_game_id("Game 37:"), 
            Ok(("", 37))
        );
    }

    #[test]
    fn parse_cube_test() {
        assert_eq!(parse_cube("12 red"), Ok(("", Cube::Red(12))));
        assert_eq!(parse_cube("39 green"), Ok(("", Cube::Green(39))));
        assert_eq!(parse_cube("7 blue"), Ok(("", Cube::Blue(7))));
        assert_eq!(parse_cube("7 yellow"), Ok(("", Cube::Other(7))));
    }

    #[test]
    fn parse_hand_test() {
        assert_eq!(parse_hand("7 red"), Ok(("", vec![Cube::Red(7)])));
        assert_eq!(parse_hand("7 red, 33 blue"), Ok(("", vec![Cube::Red(7), Cube::Blue(33)])));
    }

    #[test]
    fn parse_game_test() {
        let game1 = Game::new(1, vec![
            vec![Cube::Blue(3), Cube::Red(4)], 
            vec![Cube::Red(1), Cube::Green(2), Cube::Blue(6)],
            vec![Cube::Green(2)]
            ]);
            
        assert_eq!(
            parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Ok(("", game1))
        )
    }
}