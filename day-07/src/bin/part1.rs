use indicatif::ProgressBar;

fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard=0,
    OnePair=1,
    TwoPairs=2,
    ThreeOfAKind=3,
    FullHouse=4,
    FourOfAKind=5,
    FiveOfAKind=6,
}

fn card_to_value(card: &str) -> u32 {
    let mut value = 0;
    for c in card.chars() {
        value *= 10;
        value += match c {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => c.to_digit(10).unwrap(),
        };
    }
    return value;
}

fn is_five_of_a_kind(cards: &Vec<u32>) -> bool {
    // Assumes card list is sorted low to high
    return cards[0] == cards[4];
}

fn is_four_of_a_kind(cards: &Vec<u32>) -> bool {
    // Assumes card list is sorted low to high
    return !is_five_of_a_kind(cards) && (cards[0] == cards[3] || cards[1] == cards[4]);
}

fn is_full_house(cards: &Vec<u32>) -> bool {
    // Assumes card list is sorted low to high
    return (cards[0] != cards[4]) && ((cards[0] == cards[2] && cards[3] == cards[4]) || (cards[0] == cards[1] && cards[2] == cards[4]));
}

fn is_three_of_a_kind(cards: &Vec<u32>) -> bool {
    // Assumes card list is sorted low to high
    return !is_five_of_a_kind(cards) && !is_four_of_a_kind(cards) && (cards[0] == cards[2] || cards[1] == cards[3] || cards[2] == cards[4]);
}

fn is_two_pairs(cards: &Vec<u32>) -> bool {
    // Assumes card list is sorted low to high
    if is_five_of_a_kind(cards) || is_four_of_a_kind(cards) || is_full_house(cards) || is_three_of_a_kind(cards) {
        return false;
    } else if cards[0] == cards[1] {
        return cards[2] == cards[3] || cards[3] == cards[4];
    } else if cards[1] == cards[2] {
        return cards[3] == cards[4];
    }
    return false;
}

fn is_one_pair(cards: &Vec<u32>) -> bool {
    // Assumes card list is sorted low to high
    if is_five_of_a_kind(cards) || is_four_of_a_kind(cards) || is_full_house(cards) || is_three_of_a_kind(cards) || is_two_pairs(cards) {
        return false;
    }
    return cards[0] == cards[1] || cards[1] == cards[2] || cards[2] == cards[3] || cards[3] == cards[4];
}

fn cards_to_type(cards: &Vec<u32>) -> HandType {
    // Assumes card list is sorted low to high
    if is_five_of_a_kind(&cards) {
        return HandType::FiveOfAKind;
    } else if is_four_of_a_kind(&cards) {
        return HandType::FourOfAKind;
    } else if is_full_house(&cards) {
        return HandType::FullHouse;
    } else if is_three_of_a_kind(&cards) {
        return HandType::ThreeOfAKind;
    } else if is_two_pairs(&cards) {
        return HandType::TwoPairs;
    } else if is_one_pair(&cards) {
        return HandType::OnePair;
    }
    return HandType::HighCard;
}

fn hand_to_cards(hand: &str) -> Vec<u32> {
    let card_chars = hand.split("").collect::<Vec<&str>>();
    let mut cards: Vec<u32> = card_chars.iter().filter(|c| c.len() != 0).map(|c| card_to_value(c)).collect();
    cards.sort();
    return cards;
}

fn hand_to_value(hand: &str) -> u32 {
    let cards = hand_to_cards(hand);

    for card in cards.iter() {
        println!("{} ", card);
    }

    return 0;
}

fn first_is_higher(first_hand: &str, second_hand: &str) -> bool {
    // Convert hands to sorted card lists
    let first = hand_to_cards(first_hand);
    let second = hand_to_cards(second_hand);

    // First compare hand types
    let first_type = cards_to_type(&first);
    let second_type = cards_to_type(&second);

    if first_type > second_type {
        //println!("{} beats {} due to type", first_hand, second_hand);
        return true;
    } else if first_type < second_type {
        //println!("{} beats {} due to type", second_hand, first_hand);
        return false;
    }

    // Then compare card values
    let first_bytes = first_hand.as_bytes();
    let second_bytes = second_hand.as_bytes();
    for i in 0..first_bytes.len() {
        let first_char = (first_bytes[i] as char).to_string();
        let second_char = (second_bytes[i] as char).to_string();
        if card_to_value(&first_char) > card_to_value(&second_char) {
            //println!("{} beats {} due to value at pos {}, {} > {}", first_hand, second_hand, i, first_char, second_char);
            return true;
        } else if card_to_value(&first_char) < card_to_value(&second_char) {
            //println!("{} beats {} due to value at pos {}, {} < {}", second_hand, first_hand, i, first_char, second_char);
            return false;
        }
    }

    println!("Tie between {} and {}", first_hand, second_hand);
    return false;
}

fn part1(input: &str) -> String {
    let mut hands: Vec<&str> = Vec::new();
    let mut bids: Vec<u32> = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(" ").collect::<Vec<&str>>();
        let hand = parts[0];
        let bid = parts[1].parse::<u32>().unwrap();

        hands.push(hand);
        bids.push(bid);
    }

    let mut wins: Vec<u32> = vec![0; hands.len()];
    let bar = ProgressBar::new(hands.len() as u64);
    for i in 0..hands.len()-1 {
        for j in i+1..hands.len() {
            if first_is_higher(hands[i], hands[j]) {
                wins[i] += 1;
                //println!("{} beats {}", hands[i], hands[j]);
            } else {
                wins[j] += 1;
                //println!("{} beats {}", hands[j], hands[i]);
            }
        }
        bar.inc(1);
    }
    bar.finish();

    let mut sum = 0;
    for i in 0..hands.len() {
        println!("{} won {} times, rank = {} -> score {}", hands[i], wins[i], wins[i] + 1, (wins[i] + 1) * bids[i]);
        sum += (wins[i] + 1) * bids[i];
    }

    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn card_to_value_test() {
        assert_eq!(card_to_value("A"), 14);
        assert_eq!(card_to_value("T"), 10); 
        assert_eq!(card_to_value("J"), 11); 
        assert_eq!(card_to_value("Q"), 12);
        assert_eq!(card_to_value("K"), 13);
        assert_eq!(card_to_value("9"), 9);
        assert_eq!(card_to_value("8"), 8);
        assert_eq!(card_to_value("7"), 7);
        assert_eq!(card_to_value("6"), 6);
        assert_eq!(card_to_value("5"), 5);
        assert_eq!(card_to_value("4"), 4);
        assert_eq!(card_to_value("3"), 3);
        assert_eq!(card_to_value("2"), 2);
    }

    #[test]
    fn is_five_of_a_kind_test() {
        // Cards are sorted from lowest to highest
        assert_eq!(is_five_of_a_kind(&vec![14, 14, 14, 14, 14]), true);
        assert_eq!(is_five_of_a_kind(&vec![13, 14, 14, 14, 14]), false);
        assert_eq!(is_five_of_a_kind(&vec![13, 13, 14, 14, 14]), false);
        assert_eq!(is_five_of_a_kind(&vec![13, 13, 13, 14, 14]), false);
        assert_eq!(is_five_of_a_kind(&vec![13, 13, 13, 13, 14]), false);
        assert_eq!(is_five_of_a_kind(&vec![13, 13, 13, 13, 13]), true);
    }

    #[test]
    fn is_four_of_a_kind_test() {
        // Cards are sorted from lowest to highest
        assert_eq!(is_four_of_a_kind(&vec![14, 14, 14, 14, 14]), false);
        assert_eq!(is_four_of_a_kind(&vec![13, 14, 14, 14, 14]), true);
        assert_eq!(is_four_of_a_kind(&vec![13, 13, 14, 14, 14]), false);
        assert_eq!(is_four_of_a_kind(&vec![13, 13, 13, 14, 14]), false);
        assert_eq!(is_four_of_a_kind(&vec![13, 13, 13, 13, 14]), true);
        assert_eq!(is_four_of_a_kind(&vec![13, 13, 13, 13, 13]), false);
    }

    #[test]
    fn is_full_house_test() {
        // Cards are sorted from lowest to highest
        assert_eq!(is_full_house(&vec![14, 14, 14, 14, 14]), false);
        assert_eq!(is_full_house(&vec![13, 14, 14, 14, 14]), false);
        assert_eq!(is_full_house(&vec![13, 13, 14, 14, 14]), true);
        assert_eq!(is_full_house(&vec![13, 13, 13, 14, 14]), true);
        assert_eq!(is_full_house(&vec![13, 13, 13, 13, 14]), false);
        assert_eq!(is_full_house(&vec![13, 13, 13, 13, 13]), false);
    }

    #[test]
    fn is_three_of_a_kind_test() {
        // Cards are sorted from lowest to highest
        assert_eq!(is_three_of_a_kind(&vec![14, 14, 14, 14, 14]), false);
        assert_eq!(is_three_of_a_kind(&vec![13, 14, 14, 14, 14]), false);
        assert_eq!(is_three_of_a_kind(&vec![13, 13, 14, 14, 14]), true);
        assert_eq!(is_three_of_a_kind(&vec![13, 13, 13, 14, 14]), true);
        assert_eq!(is_three_of_a_kind(&vec![13, 13, 13, 13, 14]), false);
        assert_eq!(is_three_of_a_kind(&vec![13, 13, 13, 13, 13]), false);
        assert_eq!(is_three_of_a_kind(&vec![12, 13, 13, 13, 14]), true);
    }

    #[test]
    fn is_two_pairs_test() {
        // Cards are sorted from lowest to highest
        assert_eq!(is_two_pairs(&vec![14, 14, 14, 14, 14]), false);
        assert_eq!(is_two_pairs(&vec![13, 14, 14, 14, 14]), false);
        assert_eq!(is_two_pairs(&vec![13, 13, 14, 14, 14]), false);
        assert_eq!(is_two_pairs(&vec![12, 12, 13, 13, 14]), true);
        assert_eq!(is_two_pairs(&vec![13, 13, 13, 13, 14]), false);
        assert_eq!(is_two_pairs(&vec![13, 13, 13, 13, 13]), false);
        assert_eq!(is_two_pairs(&vec![12, 13, 13, 13, 14]), false);
        assert_eq!(is_two_pairs(&vec![12, 13, 13, 14, 14]), true);
    }

    #[test]
    fn is_one_pair_test() {
        // Cards are sorted from lowest to highest
        assert_eq!(is_one_pair(&vec![14, 14, 14, 14, 14]), false);
        assert_eq!(is_one_pair(&vec![13, 14, 14, 14, 14]), false);
        assert_eq!(is_one_pair(&vec![13, 13, 14, 14, 14]), false);
        assert_eq!(is_one_pair(&vec![12, 12, 13, 13, 14]), false);
        assert_eq!(is_one_pair(&vec![13, 13, 13, 13, 14]), false);
        assert_eq!(is_one_pair(&vec![13, 13, 13, 13, 13]), false);
        assert_eq!(is_one_pair(&vec![12, 13, 13, 13, 14]), false);
        assert_eq!(is_one_pair(&vec![12, 13, 13, 14, 14]), false);
        assert_eq!(is_one_pair(&vec![12, 13, 14, 14, 14]), false);
        assert_eq!(is_one_pair(&vec![11, 12, 13, 13, 14]), true);
        assert_eq!(is_one_pair(&vec![11, 12, 12, 13, 14]), true);
        assert_eq!(is_one_pair(&vec![11, 11, 12, 13, 14]), true);
        assert_eq!(is_one_pair(&vec![11, 12, 13, 14, 14]), true);
        assert_eq!(is_one_pair(&vec![10, 11, 12, 13, 14]), false);
    }

    #[test]
    fn card_to_type_test() {
        // Cards are sorted from lowest to highest
        assert_eq!(cards_to_type(&vec![14, 14, 14, 14, 14]), HandType::FiveOfAKind);
        assert_eq!(cards_to_type(&vec![13, 14, 14, 14, 14]), HandType::FourOfAKind);
        assert_eq!(cards_to_type(&vec![13, 13, 14, 14, 14]), HandType::FullHouse);
        assert_eq!(cards_to_type(&vec![12, 12, 13, 13, 14]), HandType::TwoPairs);
        assert_eq!(cards_to_type(&vec![13, 13, 13, 13, 14]), HandType::FourOfAKind);
        assert_eq!(cards_to_type(&vec![13, 13, 13, 13, 13]), HandType::FiveOfAKind);
        assert_eq!(cards_to_type(&vec![12, 13, 13, 13, 14]), HandType::ThreeOfAKind);
        assert_eq!(cards_to_type(&vec![12, 13, 13, 14, 14]), HandType::TwoPairs);
        assert_eq!(cards_to_type(&vec![11, 12, 13, 13, 14]), HandType::OnePair);
        assert_eq!(cards_to_type(&vec![11, 12, 12, 13, 14]), HandType::OnePair);
        assert_eq!(cards_to_type(&vec![11, 11, 12, 13, 14]), HandType::OnePair);
        assert_eq!(cards_to_type(&vec![11, 12, 13, 14, 14]), HandType::OnePair);
        assert_eq!(cards_to_type(&vec![10, 11, 12, 13, 14]), HandType::HighCard);
    }

    #[test]
    fn hand_to_cards_test() {
        assert_eq!(hand_to_cards("32T3K"), vec![2, 3, 3, 10, 13]);
    }

    #[test]
    fn hand_to_value_test() {
        assert_eq!(hand_to_value("32T3K"), 0);
    }

    #[test]
    fn first_is_higher_test() {
        assert_eq!(first_is_higher("33332", "2AAAA"), true);
        assert_eq!(first_is_higher("2AAAA", "33332"), false);
        assert_eq!(first_is_higher("77888", "77788"), true);
        assert_eq!(first_is_higher("77788", "77888"), false);
        // From test set
        assert_eq!(first_is_higher("32T3K", "T55J5"), false);
        assert_eq!(first_is_higher("T55J5", "QQQJA"), false);
        assert_eq!(first_is_higher("KK677", "KTJJT"), true);
    }

    #[test]
    fn it_works1() {
        let result = part1(INPUT);
        assert_eq!(result, "6440".to_string());
    }
}