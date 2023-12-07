use std::{num::ParseIntError, collections::BTreeMap, cmp, fs};

fn main() {
    println!("Hello, world!");
}


const CARD: [char;13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K','A'];
const CARD2: [char;13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K','A'];
const JOKER: char = 'J';


#[derive(Debug)]
struct Hand<'a> {
    hand: &'a str,
    rank: i32,
}


impl<'a> TryFrom<&'a str> for Hand<'a> {
    type Error = ParseIntError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut split = value.split_whitespace().into_iter();
        Ok(Hand {
            hand: split.next().unwrap().clone(),
            rank: split.next().map(|d| d.parse::<i32>().unwrap()).unwrap(),
        })
    }
}


impl Hand<'_> {
    fn count_cards(&self) -> BTreeMap<char, i32> {
        let mut carddeck: BTreeMap<char, i32> = BTreeMap::new(); 
        self.hand.chars()
                 .for_each(|c| {
                        carddeck.entry(c)
                                .and_modify(|cnt| {*cnt += 1})
                                .or_insert(1);
                });
        carddeck
    }
}


fn compare_part1(a: &Hand<'_>, b: &Hand<'_>) -> cmp::Ordering {
    let cmp = a.count_cards().values().map(|v| v*v).sum::<i32>()
                            .cmp(
                                &b.count_cards().values().map(|v| v*v).sum::<i32>()
                            );

    match cmp {
        cmp::Ordering::Equal => a.hand.chars().zip(b.hand.chars())
                                            .map(|(a,b)| index_of_card(&a,&b))
                                            .skip_while(|cmp| cmp.is_eq())
                                            .next()
                                            .unwrap_or(cmp::Ordering::Equal),
        other => other,
    }
}


fn parse_card(input: &str) -> i32 {
    let mut cards : Vec<Hand> = input.lines()
                                .map(|line|Hand::try_from(line).unwrap())
                                .collect();
    cards.sort_by(compare_part1);
    (0..cards.len()).map(|n| cards.get(n).unwrap().rank * (1 + n as i32)).sum()
}


fn compare_part2(a: &Hand<'_>, b: &Hand<'_>) -> cmp::Ordering {
    let cards_a = card_recounted_for_joker(a);
    let cards_b = card_recounted_for_joker(b);

    let cmp = cards_a.values().map(|v| v*v).sum::<i32>()
                            .cmp(
                                &cards_b.values().map(|v| v*v).sum::<i32>()
                            );
    match cmp {
        cmp::Ordering::Equal => a.hand.chars().zip(b.hand.chars())
                                            .map(|(a,b)| index_of_card2(&a,&b))
                                            .skip_while(|cmp| cmp.is_eq())
                                            .next()
                                            .unwrap_or(cmp::Ordering::Equal),
        other => other,
    }

}


fn card_recounted_for_joker(hand: &Hand) -> BTreeMap<char, i32> {
    let mut card_counts = hand.count_cards();
    if let Some(&joker_count) = &card_counts.get(&JOKER) {
        if joker_count != 5 {
            let max_cards = &card_counts.iter()
                                    .filter(|e| *e.0 != JOKER)
                                    .max_by_key(|kv| kv.1)
                                    .unwrap();
            card_counts.entry(*max_cards.0).and_modify(|v| *v += joker_count);
            card_counts.remove_entry(&JOKER);
        }
    }
    return card_counts;
}


fn parse_card2(input: &str) -> i32 {
    let mut cards : Vec<Hand> = input.lines()
                                .map(|line|Hand::try_from(line).unwrap())
                                .collect();
    cards.sort_by(compare_part2);
    (0..cards.len()).map(|n| cards.get(n).unwrap().rank * (1 + n as i32)).sum()
}


fn index_of_card(a: &char, b: &char) -> std::cmp::Ordering {
    let index_a = CARD.iter().enumerate().find(|c| c.1 == a).map(|e| e.0);
    let index_b = CARD.iter().enumerate().find(|c| c.1 == b).map(|e| e.0);
    index_a.cmp(&index_b)
}


fn index_of_card2(a: &char, b: &char) -> std::cmp::Ordering {
    let index_a = CARD2.iter().enumerate().find(|c| c.1 == a).map(|e| e.0);
    let index_b = CARD2.iter().enumerate().find(|c| c.1 == b).map(|e| e.0);
    index_a.cmp(&index_b)
}


#[test]
fn example1() {
    let input =
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    // list_of_hands + bid
    // 32T3K = 1 pair = 1               3*1*1+1*2*2 = 7
    // KTJJT = 2 pair = 2 // K.cmp(T)   2*2+2*2+1*1 = 9
    // KK677 = 2 pair = 3               2*2+2*2+1*1 = 9
    // T55J5 = 3 kind = 4 // T.cmp(Q)   3*3+1*1+1*1 = 11
    // QQQJA = 3 kind = 5               3*3+1*1+1*1 = 11
    println!("sum cards: \x1b[32m{}\x1b[0m", parse_card(input));
    assert_eq!(6440, parse_card(input));
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    println!("total winnings: \x1b[32m{}\x1b[0m", parse_card(input.as_str()));
    assert_eq!(248569531, parse_card(input.as_str()));
    // < 249169463
}


#[test]
fn example2() {
    let input =
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    // list_of_hands + bid
    // J = Joker => whatever card || weakest

    // 32T3K = 1 pair = 1               3*1*1+1*2*2 = 7
    // KK677 = 2 pair = 2               2*2+2*2+1*1 = 9
    // T55J5 = 4 kind = 3                   4*4+1*1 = 17
    // QQQJA = 4 kind = 4 // Q.cmp(T)       4*4+1*1 = 17
    // KTJJT = 4 kind = 5 // K.cmp(Q)       4*4+1*1 = 17
    println!("sum cards: \x1b[32m{}\x1b[0m", parse_card2(input));
    assert_eq!(5905, parse_card2(input));
}


#[test]
fn part() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    println!("total winnings: \x1b[32m{}\x1b[0m", parse_card2(input.as_str()));
    // > 250146490
    assert_eq!(250382098, parse_card2(input.as_str()));
}
