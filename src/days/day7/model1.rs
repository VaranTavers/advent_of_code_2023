use std::{num::ParseIntError, str::FromStr};

#[derive(Clone, Copy, Debug)]
pub struct Card(pub i64);

impl Card {
    pub fn from_char(c: char) -> Card {
        match c {
            'A' => Card(14),
            'K' => Card(13),
            'Q' => Card(12),
            'J' => Card(11),
            'T' => Card(10),
            _ => Card(c.to_digit(10).unwrap() as i64),
        }
    }
}

#[derive(Debug)]
pub struct Hand {
    cards: [Card; 5],
    pub value: usize,
    pub bet: i64,
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.value != other.value {
            return false;
        }
        for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
            let cmp_res = c1.0.cmp(&c2.0);
            if cmp_res != std::cmp::Ordering::Equal {
                return false;
            }
        }
        true
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.value.cmp(&other.value) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => {
                for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                    let cmp_res = c1.0.cmp(&c2.0);
                    if cmp_res != std::cmp::Ordering::Equal {
                        return cmp_res;
                    }
                }
                std::cmp::Ordering::Equal
            }
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

impl Hand {
    fn get_value_from_cards(cards: &Vec<Card>) -> usize {
        let mut cards_num: [usize; 15] = [0; 15];
        for card in cards {
            cards_num[card.0 as usize] += 1;
        }

        let max = *cards_num.iter().max().unwrap_or(&0);
        if max == 5 || max == 4 {
            return max + 1;
        }

        if max == 3 {
            if cards_num.iter().any(|x| *x == 2) {
                return 4;
            }

            return 3;
        }

        if max == 2 {
            return cards_num.iter().filter(|x| **x == 2).count();
        }

        0
    }

    pub fn from_cards_and_bet(cards: Vec<Card>, bet: i64) -> Hand {
        Hand {
            cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
            value: Hand::get_value_from_cards(&cards),
            bet,
        }
    }
}

impl FromStr for Hand {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s.split(' ').collect::<Vec<&str>>();

        let cards = vals[0].chars().map(Card::from_char).collect::<Vec<Card>>();
        let bet = vals[1].parse::<i64>()?;

        Ok(Hand::from_cards_and_bet(cards, bet))
    }
}
