pub type Hand = Vec<Cube>;

#[derive(Debug, PartialEq)]
pub enum Cube {
    Red(u32),
    Blue(u32),
    Green(u32),
    Other(u32),
}

pub struct Rule {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
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

    pub fn is_valid(&self, rule: &Rule) -> Result<(), String> {
        for hand in &self.hands {
            validate_hand(hand, rule)?
        }

        return Ok(())
    }
}

fn validate_hand(hand: &Hand, r: &Rule) -> Result<(), String> {
    for cube in hand {
        match cube {
            Cube::Blue(x)  if x > &r.blue => return Err(format!("blue is {}, limit is {}", x, &r.blue)),
            Cube::Green(x) if x > &r.green => return Err(format!("green is {}, limit is {}", x, &r.green)),
            Cube::Red(x)   if x > &r.red => return Err(format!("red is {}, limit is {}", x, &r.red)),
            Cube::Other(_) => return Err("invalid cube!".into()),

            _ => continue
        }
    }

    return Ok(())
}

#[cfg(test)]
mod test {
    use crate::game::{validate_hand, Rule, Cube};

    #[test]
    fn validate_hand_test() {
        let rule = Rule{
            blue: 10,
            green: 10,
            red: 10,
        };

        assert_eq!(
            validate_hand(&vec![Cube::Red(3), Cube::Green(7), Cube::Blue(10)], &rule), 
            Ok(())
        );

        assert_eq!(
            validate_hand(&vec![Cube::Green(11), Cube::Green(7)], &rule), 
            Err(String::from("red is 11, limit is 10"))
        );
    }
}