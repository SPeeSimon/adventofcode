use std::{fs, str::FromStr, num::ParseIntError, collections::BTreeMap};


#[derive(Debug, Eq, PartialEq, Hash, PartialOrd)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    numbers_you_have: Vec<i32>,
}


impl FromStr for Card {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_part, content_part) = s.split_once(":").unwrap();
        let (winning_numbers_part, numbers_you_have_part) = content_part.split_once(" | ").unwrap();
        
        let card_id = id_part.split_whitespace().last().unwrap().parse::<i32>();
        let winnings = winning_numbers_part.split_whitespace()
                                           .map(|d| d.parse::<i32>().unwrap())
                                           .collect();
        let numbers_you_have = numbers_you_have_part.split_whitespace()
                                                    .map(|d| d.parse::<i32>().unwrap())
                                                    .collect();
        // "Card {id}: {list of nr} | {list of nr}";
        Ok(Card {
            id: card_id.unwrap(),
            winning_numbers: winnings,
            numbers_you_have: numbers_you_have,
        })
    }
}


impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}


impl Card {

    fn matching(&self) -> impl Iterator<Item=&i32> + '_ {
        self.numbers_you_have.iter().filter(|nr| self.winning_numbers.contains(nr))
    }

    fn count_winnings(&self) -> i32 {
        let mut points = 0;
        for _n in self.matching() {
            if points == 0 {
                points += 1;
            } else {
                points *= 2;
            }
        }
        return points;
    }
}


fn scratchcards(input: &str) -> i32 {
    let cards: Vec<Card> = input.lines().map(|line| Card::from_str(line).unwrap()).collect();
    let mut carddeck: BTreeMap<Card, i32> = BTreeMap::from_iter(cards.into_iter().map(|card| (card, 1)));
    let mut total_scratchcards = 0;

    while carddeck.values().any(|cnt| *cnt > 0) {
        let mut card_iter = carddeck.iter_mut().skip_while(|e| *e.1 == 0);

        if let Some((current_card, current_card_count)) = card_iter.next() {
            for _n in current_card.matching() {
                // println!("current {:?} @ {:?} = + {}", &current_card, n, current_card_count);
                let (_, next_card_count) = card_iter.next().unwrap();
                *next_card_count += *current_card_count;
            }
            total_scratchcards += *current_card_count;
            *current_card_count = 0;
        }
    }

    return total_scratchcards;
}


#[test]
fn example1() {

    let input = 
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    // card 1 has 8 points
    // match: 48,  83,  17,  86
    // result: 1 -> 2 -> 4 -> 8

    let mut cards = input.lines().map(|line| Card::from_str(line).unwrap());
    let mut total_points = 0;

    let mut card = cards.next().unwrap();
    println!("{:?} = {}", card, card.count_winnings());
    assert_eq!(1, card.id);
    assert_eq!(8, card.count_winnings());
    total_points += card.count_winnings();

    card = cards.next().unwrap();
    println!("{:?} = {}", card, card.count_winnings());
    assert_eq!(2, card.id);
    assert_eq!(2, card.count_winnings());
    total_points += card.count_winnings();

    card = cards.next().unwrap();
    println!("{:?} = {}", card, card.count_winnings());
    assert_eq!(3, card.id);
    assert_eq!(2, card.count_winnings());
    total_points += card.count_winnings();

    card = cards.next().unwrap();
    println!("{:?} = {}", card, card.count_winnings());
    assert_eq!(4, card.id);
    assert_eq!(1, card.count_winnings());
    total_points += card.count_winnings();

    card = cards.next().unwrap();
    println!("{:?} = {}", card, card.count_winnings());
    assert_eq!(5, card.id);
    assert_eq!(0, card.count_winnings());
    total_points += card.count_winnings();

    card = cards.next().unwrap();
    println!("{:?} = {}", card, card.count_winnings());
    assert_eq!(6, card.id);
    assert_eq!(0, card.count_winnings());
    total_points += card.count_winnings();

    println!("total: {}", total_points);
    assert_eq!(13, total_points);
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let total_points: i32 = input.lines().map(|line| Card::from_str(line).unwrap()).map(|c| c.count_winnings()).sum();
    println!("total: {}", total_points);
}


#[test]
fn example2() {
    let input = 
    "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    // card 1: 4 matching => copy of card: 2, 3, 4, 5
    // card 2: 2 matching => copy of card: 3, 4 (2x)
    // card 2 (win from previous): 2 matching => copy of card: 3, 4
    // card 3 (1+1+2): 2 matching => copy of card: 4 5 (4x)
    // card 4 (8x): 1 matching => copy of card: 5 (8x)
    // card 5 (15x): 0 matching => copy of card: -
    // card 6 (1x): 0 matching => copy of card: -
    // cards: 1 (1x), 2 (2x), 3 (4x), 4 (8x), 5 (14x), 6 (1x) = 30 cards
    // how many total scratchcards
    let total_scratchcards = scratchcards(input);
    println!("total cards: {}", total_scratchcards);
    assert_eq!(30, total_scratchcards);
}

#[test]
fn part2() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let total_scratchcards = scratchcards(input.as_str());
    println!("total cards: {}", total_scratchcards);
}
