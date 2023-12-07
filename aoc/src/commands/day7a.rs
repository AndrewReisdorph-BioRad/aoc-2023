use std::path::PathBuf;

use clap::Parser;

use crate::utils::slurp_bytes;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day7a {
    #[clap(long, short)]
    input: PathBuf,
}

pub struct Solver {
    bytes: Vec<u8>,
    read_idx: usize,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandKind {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    kind: HandKind,
    cards: [u8; 5],
    bet: u32,
}

impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.kind == other.kind {
            for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
                if card1 != card2 {
                    if card1 == &b'A' {
                        return std::cmp::Ordering::Greater;
                    } else if card2 == &b'A' {
                        return std::cmp::Ordering::Less;
                    }

                    if card1 == &b'K' {
                        return std::cmp::Ordering::Greater;
                    } else if card2 == &b'K' {
                        return std::cmp::Ordering::Less;
                    }

                    if card1 == &b'Q' {
                        return std::cmp::Ordering::Greater;
                    } else if card2 == &b'Q' {
                        return std::cmp::Ordering::Less;
                    }

                    if card1 == &b'J' {
                        return std::cmp::Ordering::Greater;
                    } else if card2 == &b'J' {
                        return std::cmp::Ordering::Less;
                    }

                    if card1 == &b'T' {
                        return std::cmp::Ordering::Greater;
                    } else if card2 == &b'T' {
                        return std::cmp::Ordering::Less;
                    }

                    return card1.cmp(card2);
                }
            }
            unreachable!("Hands are equal")
        } else {
            let other_kind = other.kind as u8;
            (self.kind as u8).cmp(&other_kind)
        }

        // let other_kind = other.kind as u8;
        // (self.kind as u8).cmp(&other_kind)
    }
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solver {
    fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            read_idx: 0,
        }
    }

    fn get_hand_kind(cards: &[u8; 5]) -> HandKind {
        let mut card_map = [0; 255];

        for card in cards {
            card_map[*card as usize] += 1;
        }

        let mut pair_found: bool = false;
        let mut three_of_a_kind_found: bool = false;

        for count in card_map.iter() {
            match *count {
                0 => (),
                1 => (),
                2 => {
                    if pair_found {
                        return HandKind::TwoPair;
                    } else if three_of_a_kind_found {
                        return HandKind::FullHouse;
                    }
                    pair_found = true;
                },
                3 => {
                    if pair_found {
                        return HandKind::FullHouse;
                    } else {
                        three_of_a_kind_found = true;
                    }
                },
                4 => return HandKind::FourOfAKind,
                5 => return HandKind::FiveOfAKind,
                _ => unreachable!(),
            }
        }

        match (pair_found, three_of_a_kind_found) {
            (true, true) => return HandKind::FullHouse,
            (true, false) => return HandKind::OnePair,
            (false, true) => return HandKind::ThreeOfAKind,
            (false, false) => (),
        }

        HandKind::HighCard
    }

    fn read_next_hand(&mut self) -> Option<Hand> {
        if self.read_idx == self.bytes.len() {
            return None;
        }

        // Read cards
        let cards: [u8; 5] = self.bytes[self.read_idx..self.read_idx + 5].try_into().unwrap();
        self.read_idx += 5;

        let kind = Self::get_hand_kind(&cards);

        // Read bet
        let bet = self.read_next_number().unwrap();
        self.read_idx += 1;

        Some(Hand {
            kind,
            cards,
            bet,
        })
    }

    fn read_next_number(&mut self) -> Option<u32> {
        if self.read_idx >= self.bytes.len() || self.bytes[self.read_idx] == b'\n' {
            return None;
        }

        while self.bytes[self.read_idx] == b' ' {
            self.read_idx += 1;
        }

        let mut num = 0;
        while self.bytes[self.read_idx].is_ascii_digit() {
            num *= 10;
            num += (self.bytes[self.read_idx] - b'0') as u32;
            self.read_idx += 1;
        }

        while self.bytes[self.read_idx] == b' ' {
            self.read_idx += 1;
        }
        // *read_idx += 1;
        Some(num)
    }

    pub fn solve(&mut self) -> u64 {
        let mut hands = Vec::new();

        while let Some(hand) = self.read_next_hand() {
            hands.push(hand);
        }

        hands.sort();

        let mut winnings: u64 = 0;

        for (idx, hand) in hands.iter().enumerate() {
            // println!("{:#?}", hand);
            winnings += hand.bet as u64 * (idx + 1) as u64;
        }
        // five_of_a_kind.iter().sort_by(|a, b| b.bet.cmp(&a.bet));
        // println!("{:#?}", hands);
        winnings
    }
}

impl CommandImpl for Day7a {
    fn main(&mut self) -> Result<(), DynError> {
        let bytes = slurp_bytes(self.input.as_path()).unwrap();
        let answer = Solver::new(bytes).solve();
        println!("7A: {answer}");

        Ok(())
    }
}
