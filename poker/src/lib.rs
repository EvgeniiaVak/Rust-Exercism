/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // assign a rank to each hand
    let mut hands: Vec<Hand> = hands.iter().map(|hand| Hand::new(hand)).collect();

    // find the highest rank
    hands.sort_by_key(|hand| (hand.hand_rank, hand.card_ranks));
    let (highest_hand_rank, highest_card_ranks) = (
        hands.last().unwrap().hand_rank,
        hands.last().unwrap().card_ranks,
    );

    // both hands have two pairs
    if highest_hand_rank == HandRank::TwoPair && hands[1].hand_rank == HandRank::TwoPair {
        unimplemented!()
        // highest ranked pair wins

        // with the same highest ranked pair, tie goes to low pair
    }

    // return the hands with the highest rank
    hands
        .into_iter()
        .filter(|hand| (hand.hand_rank, hand.card_ranks) == (highest_hand_rank, highest_card_ranks))
        .map(|hand| hand.cards)
        .collect()
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

/// https://en.wikipedia.org/wiki/List_of_poker_hands
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    hand_rank: HandRank,
    card_ranks: (CardRank, CardRank, CardRank, CardRank, CardRank),
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
        let mut card_ranks = cards
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
            .collect::<Vec<CardRank>>();

        card_ranks.sort();

        let mut same_card_rank_counts = vec![1];
        card_ranks.windows(2).for_each(|window| {
            if window[0] == window[1] {
                let count = same_card_rank_counts.last_mut().unwrap();
                *count += 1;
            } else {
                same_card_rank_counts.push(1);
            }
        });
        same_card_rank_counts.sort();

        let mut hand_rank = match same_card_rank_counts.as_slice() {
            [1, 1, 1, 1, 1] => HandRank::HighCard,
            [1, 1, 1, 2] => HandRank::OnePair,
            [1, 2, 2] => HandRank::TwoPair,
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
            card_ranks: (
                card_ranks[0],
                card_ranks[1],
                card_ranks[2],
                card_ranks[3],
                card_ranks[4],
            ),
        }
    }
}
