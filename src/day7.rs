use std::{cmp::Ordering, hash::Hash};

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use tap::Pipe;

fn main() -> Result<()> {
    let input = include_str!("../inputs/day7.txt");
    println!("Part 1 = {}", part1(input)?);
    println!("Part 2 = {}", part2(input)?);
    Ok(())
}

#[derive(PartialEq, Eq, Hash)]
struct Card(char);

impl Card {
    fn strength(&self) -> u32 {
        match self.0 {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            'j' => 0, //lowercase j represents a joker
            c => c.to_digit(10).expect("Unexpected card symbol"),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let Some(most_common_card) = self
            .cards
            .iter()
            .filter(|card| **card != Card('j'))
            .pipe(mode)
        else {
            return HandType::FiveOfAKind; //edge case of all jokers
        };

        let counts = self
            .cards
            .iter()
            .update(|card| {
                if *card == &Card('j') {
                    *card = most_common_card
                }
            })
            .counts();

        match counts.len() {
            1 => HandType::FiveOfAKind,
            2 if counts.values().contains(&4) => HandType::FourOfAKind,
            2 => HandType::FullHouse,
            3 if counts.values().contains(&3) => HandType::ThreeOfAKind,
            3 => HandType::TwoPair,
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn cmp_cards(&self, other: &Hand) -> Ordering {
        self.cards
            .iter()
            .zip(other.cards.iter())
            .map(|(card, other_card)| card.cmp(other_card))
            .find(|ordering| !ordering.is_eq())
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp_cards(&other).is_eq()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then_with(|| self.cmp_cards(other))
    }
}

//returns most common element or None if the iterator is empty
fn mode<E: Hash + Eq>(iter: impl Iterator<Item = E>) -> Option<E> {
    iter.counts()
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(elem, _)| elem)
}

fn parse_line(line: &str) -> Result<(Hand, u32)> {
    let (cards, bid) = line
        .split_whitespace()
        .collect_tuple()
        .context("Error: input line must have 2 tokens")?;
    let bid = bid.parse()?;
    let cards = cards
        .chars()
        .map(Card)
        .collect_vec()
        .try_into()
        .map_err(|_| anyhow!("Error: a hand must be 5 cards"))?;
    let hand = Hand { cards };
    Ok((hand, bid))
}

fn inject_jokers(hand: &mut Hand) {
    for card in &mut hand.cards {
        if *card == Card('J') {
            *card = Card('j');
        }
    }
}

fn calculate_winnings(pairs: impl Iterator<Item = (Hand, u32)>) -> u32 {
    pairs
        .sorted_unstable_by(|pair, other| pair.0.cmp(&other.0))
        .map(|(_, bid)| bid)
        .enumerate()
        .map(|(i, bid)| (i as u32 + 1) * bid)
        .sum()
}

fn part1(input: &str) -> Result<u32> {
    input
        .lines()
        .map(parse_line)
        .process_results(|pairs| calculate_winnings(pairs))
}

fn part2(input: &str) -> Result<u32> {
    input.lines().map(parse_line).process_results(|pairs| {
        pairs
            .update(|(hand, _)| inject_jokers(hand))
            .pipe(calculate_winnings)
    })
}
