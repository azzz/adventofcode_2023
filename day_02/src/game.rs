pub type Hand = Vec<Cube>;

#[derive(Debug, PartialEq)]
pub enum Cube {
    Red(u32),
    Blue(u32),
    Green(u32),
    Other(u32),
}

// (red, green, blue)
#[derive(Debug, PartialEq)]
pub struct Set(pub u32, pub u32, pub u32);

impl From<&Hand> for Set {
    fn from(hand: &Hand) -> Self {
        let mut reds = 0;
        let mut greens = 0;
        let mut blues = 0;

        for cube in hand {
            match cube {
                Cube::Red(v) => reds += v,
                Cube::Green(v) => greens += v,
                Cube::Blue(v) => blues += v,
                _ => (),
            }
        }

        Set(reds, greens, blues)
    }
}

impl From<(u32, &str)> for Cube {
    fn from(value: (u32, &str)) -> Self {
        match value {
            (v, "blue") => Cube::Blue(v),
            (v, "red") => Cube::Red(v),
            (v, "green") => Cube::Green(v),
            
            (v, _) => Cube::Other(v),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Game {
    id: u32,
    hands: Vec<Hand>
}

impl Game {
    pub fn new(id: u32, hands: Vec<Hand>) -> Self {
        Game { id, hands }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn is_valid(&self, rule: &Set) -> Result<(), String> {
        for hand in &self.hands {
            validate_hand(hand, rule)?
        }

        return Ok(())
    }

    pub fn power(&self) -> u32 {
        let set = self.min_set();
        set.0 * set.1 * set.2
    }

    pub fn min_set(&self) -> Set {
        let mut min = Set(0, 0, 0);

        for hand in &self.hands {
            let set: Set = hand.into();

            if set.0 > min.0 { min.0 = set.0 }
            if set.1 > min.1 { min.1 = set.1 }
            if set.2 > min.2 { min.2 = set.2 }
        }

        min
    }
}

fn validate_hand(hand: &Hand, r: &Set) -> Result<(), String> {
    for cube in hand {
        match cube {
            Cube::Red(x)   if x > &r.0 => return Err(format!("red is {}, limit is {}", x, &r.0)),
            Cube::Green(x) if x > &r.1 => return Err(format!("green is {}, limit is {}", x, &r.1)),
            Cube::Blue(x)  if x > &r.2 => return Err(format!("blue is {}, limit is {}", x, &r.2)),
            Cube::Other(_) => return Err("invalid cube!".into()),

            _ => continue
        }
    }

    return Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_hand_test() {
        let rule = Set(10, 10, 10);

        assert_eq!(
            validate_hand(&vec![Cube::Red(3), Cube::Green(7), Cube::Blue(10)], &rule), 
            Ok(())
        );

        assert_eq!(
            validate_hand(&vec![Cube::Green(11), Cube::Green(7)], &rule), 
            Err(String::from("red is 11, limit is 10"))
        );
    }

    #[test]
    fn min_set_test() {
        let hand1: Hand = vec![Cube::Blue(3), Cube::Red(4)];
        let hand2: Hand = vec![Cube::Red(1), Cube::Green(2), Cube::Blue(6)];
        let hand3: Hand = vec![Cube::Green(2)];
        let game1 = Game::new(0, vec![hand1, hand2, hand3]);

        assert_eq!(game1.min_set(), Set(4, 2, 6));
    }

    #[test]
    fn power_test() {
        let hand1: Hand = vec![Cube::Blue(3), Cube::Red(4)];
        let hand2: Hand = vec![Cube::Red(1), Cube::Green(2), Cube::Blue(6)];
        let hand3: Hand = vec![Cube::Green(2)];
        let game1 = Game::new(0, vec![hand1, hand2, hand3]);

        assert_eq!(game1.power(), 48);
    }
}