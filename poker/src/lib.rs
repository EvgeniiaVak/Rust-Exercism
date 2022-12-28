use std::{cmp::Ordering, collections::BTreeMap, collections::BinaryHeap};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands: BinaryHeap<Hand> = hands.iter().map(|hand| Hand::new(hand)).collect();

    let mut winning = vec![hands.pop().unwrap()];
    while let Some(hand) = hands.pop() {
        if hand < winning[0] {
            break;
        }
        winning.push(hand);
    }

    winning.into_iter().map(|hand| hand.cards).collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum CardRank {
    AceLow,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct PairsKickerSplit {
    pairs: [CardRank; 2],
    kicker: CardRank,
}

impl PairsKickerSplit {
    fn new(card_rank_counts: BTreeMap<CardRank, u8>) -> Self {
        let pairs: [CardRank; 2] = card_rank_counts
            .iter()
            .filter(|(_, &count)| count == 2)
            .map(|(&card_rank, _)| card_rank)
            .collect::<Vec<CardRank>>()
            .try_into()
            .unwrap();

        let kicker = card_rank_counts
            .iter()
            .find(|(_, &count)| count == 1)
            .map(|(&card_rank, _)| card_rank)
            .unwrap();

        Self { pairs, kicker }
    }
}

impl PartialOrd for PairsKickerSplit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.pairs == other.pairs {
            return self.kicker.partial_cmp(&other.kicker);
        }
        if self.pairs[1] == other.pairs[1] {
            return self.pairs[0].partial_cmp(&other.pairs[0]);
        }

        self.pairs[1].partial_cmp(&other.pairs[1])
    }
}

impl Ord for PairsKickerSplit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

/// https://en.wikipedia.org/wiki/List_of_poker_hands
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair(PairsKickerSplit),
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    FiveOfAKind,
}

#[derive(Debug, Clone, Ord)]
struct Hand<'a> {
    cards: &'a str,
    hand_rank: HandRank,
    card_ranks: [CardRank; 5],
}

fn is_straight(card_ranks: &mut [CardRank]) -> bool {
    for i in 0..card_ranks.len() - 1 {
        let a = card_ranks[i] as u8;
        let b = card_ranks[i + 1] as u8;
        if a + 1 != b {
            if i == 3 && card_ranks[i + 1] == CardRank::Ace && card_ranks[0] == CardRank::Two {
                card_ranks[i + 1] = CardRank::AceLow;
                return true;
            } else {
                return false;
            }
        }
    }
    true
}

impl<'a> Hand<'a> {
    fn new(cards: &'a str) -> Hand<'a> {
        let mut card_ranks: [CardRank; 5] = cards
            .split_whitespace()
            .map(|card| match &card[0..1] {
                "2" => CardRank::Two,
                "3" => CardRank::Three,
                "4" => CardRank::Four,
                "5" => CardRank::Five,
                "6" => CardRank::Six,
                "7" => CardRank::Seven,
                "8" => CardRank::Eight,
                "9" => CardRank::Nine,
                "1" => CardRank::Ten,
                "J" => CardRank::Jack,
                "Q" => CardRank::Queen,
                "K" => CardRank::King,
                "A" => CardRank::Ace,
                _ => panic!("Invalid card rank"),
            })
            .collect::<Vec<CardRank>>()
            .try_into()
            .unwrap();

        card_ranks.sort();

        let mut card_rank_counts = BTreeMap::new();
        for card_rank in card_ranks.iter() {
            *card_rank_counts.entry(*card_rank).or_insert(0) += 1;
        }

        let mut counts = card_rank_counts.values().collect::<Vec<&u8>>();
        counts.sort();

        let mut hand_rank = match counts.as_slice() {
            [1, 1, 1, 1, 1] => HandRank::HighCard,
            [1, 1, 1, 2] => HandRank::OnePair,
            [1, 2, 2] => HandRank::TwoPair(PairsKickerSplit::new(card_rank_counts)),
            [1, 1, 3] => HandRank::ThreeOfAKind,
            [2, 3] => HandRank::FullHouse,
            [1, 4] => HandRank::FourOfAKind,
            [5] => HandRank::FiveOfAKind,
            _ => panic!("Invalid card ranks"),
        };

        let is_flush = cards
            .split_whitespace()
            .map(|card| card.chars().last().unwrap())
            .collect::<Vec<char>>()
            .windows(2)
            .all(|window| window[0] == window[1]);

        let is_straight = is_straight(card_ranks.as_mut_slice());

        match (is_straight, is_flush) {
            (true, true) => hand_rank = HandRank::StraightFlush,
            (true, false) => hand_rank = HandRank::Straight,
            (false, true) => hand_rank = HandRank::Flush,
            _ => (),
        }

        Hand {
            cards,
            hand_rank,
            card_ranks,
        }
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.hand_rank == other.hand_rank && self.card_ranks == other.card_ranks
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_rank == other.hand_rank {
            return Some(self.card_ranks.cmp(&other.card_ranks));
        }

        Some(self.hand_rank.cmp(&other.hand_rank))
    }
}

impl<'a> Eq for Hand<'a> {}
