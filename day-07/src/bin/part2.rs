use indicatif::ProgressBar;

fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
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
            'J' => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => c.to_digit(10).unwrap(),
        };
    }
    return value;
}

fn get_num_jokers(cards: &Vec<u32>) -> u32 {
    let mut num_jokers = 0;
    for card in cards {
        if *card == 1 {
            num_jokers += 1;
        }
    }
    return num_jokers;
}

fn is_five_of_a_kind(cards: &Vec<u32>) -> bool {
    // Assumes card list is sorted low to high
    let num_jokers = get_num_jokers(&cards);
    if num_jokers == 0 {
        return cards[0] == cards[4]
    } else if num_jokers == 1 {
        return cards[1] == cards[4];
    } else if num_jokers == 2 {
        return cards[2] == cards[4];
    } else if num_jokers == 3 {
        return cards[3] == cards[4];
    } else if num_jokers >= 4 {
        return true;
    }
    return false;
}

fn is_four_of_a_kind(cards: &Vec<u32>) -> bool {
    // Assumes card list is sorted low to high
    let num_jokers = get_num_jokers(&cards);

    if num_jokers == 0 {
        return cards[0] == cards[3] || cards[1] == cards[4];
    } else if num_jokers == 1 {
        return cards[1] == cards[3] || cards[2] == cards[4];
    } else if num_jokers == 2 {
        return cards[2] == cards[3] || cards[3] == cards[4];
    } else if num_jokers == 3 {
        return cards[3] != cards[4];
    // This is technically a full house, but it's better as five of a kind
    //} else if num_jokers == 4 {
    //    return true;
    }
    return false;
}

fn is_full_house(cards: &Vec<u32>) -> bool {
    // Assumes card list is sorted low to high
    let num_jokers = get_num_jokers(&cards);

    if num_jokers == 0 {
        return (cards[0] != cards[4]) && ((cards[0] == cards[2] && cards[3] == cards[4]) || (cards[0] == cards[1] && cards[2] == cards[4]))
    } else if num_jokers == 1 {
        return cards[1] == cards[2] && cards[3] == cards[4];
    } else if num_jokers == 2 {
        return cards[2] == cards[3] || cards[3] == cards[4];
    // This is technically a full house, but it's better as four of a kind
    //} else if num_jokers == 3 && cards[3] != cards[4] {
    //    return true;
    }

    // assert_eq!(is_full_house(&vec![1, 1, 1, 13, 14]), true);
    
    return false;
}

fn is_three_of_a_kind(cards: &Vec<u32>) -> bool {
    // Assumes card list is sorted low to high
    let num_jokers = get_num_jokers(&cards);

    if num_jokers == 0 {
        return cards[0] == cards[2] || cards[1] == cards[3] || cards[2] == cards[4];
    } else if num_jokers == 1 {
        return cards[1] == cards[2] || cards[2] == cards[3] || cards[3] == cards[4];
    } else if num_jokers == 2 {
        return true;
    }
    return false;
}

fn is_two_pairs(cards: &Vec<u32>) -> bool {
    // Assumes card list is sorted low to high
    let num_jokers = get_num_jokers(&cards);

    if cards[0] == cards[1] {
        return cards[2] == cards[3] || cards[3] == cards[4];
    } else if num_jokers == 0 && cards[1] == cards[2] {
        return cards[3] == cards[4];
    } else if num_jokers == 1 && (cards[1] == cards[2] || cards[2] == cards[3] || cards[3] == cards[4]) {
        return true;
    } else if num_jokers == 2 {
        return true;
    }
    return false;
}

fn is_one_pair(cards: &Vec<u32>) -> bool {
    // Assumes card list is sorted low to high
    let num_jokers = get_num_jokers(&cards);

    return num_jokers == 1 || cards[0] == cards[1] || cards[1] == cards[2] || cards[2] == cards[3] || cards[3] == cards[4];
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

fn first_is_higher(first_hand: &str, second_hand: &str) -> bool {
    // Convert hands to sorted card lists
    let first = hand_to_cards(first_hand);
    let second = hand_to_cards(second_hand);

    // First compare hand types
    let first_type = cards_to_type(&first);
    let second_type = cards_to_type(&second);
    //println!("{} is a {:?}, {} is a {:?}", first_hand, first_type, second_hand, second_type);

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

fn part2(input: &str) -> String {
    let mut hands: Vec<&str> = Vec::new();
    let mut bids: Vec<u32> = Vec::new();
    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
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
        //println!("{} won {} times, rank = {} -> score {}", hands[i], wins[i], wins[i] + 1, (wins[i] + 1) * bids[i]);
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
        assert_eq!(card_to_value("J"), 1); 
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
        assert_eq!(is_five_of_a_kind(&vec![1, 13, 13, 13, 13]), true);
    }

    #[test]
    fn is_four_of_a_kind_test() {
        // Cards are sorted from lowest to highest

        // 1,  2, 2,  2, 3
        // 1,  2,  3, 3, 3
        // 1, 1,  2, 2,  3
        // 1, 1,  2,  3, 3
        // 1, 1, 1,  2,  3
        assert_eq!(is_four_of_a_kind(&vec![14, 14, 14, 14, 14]), true);
        assert_eq!(is_four_of_a_kind(&vec![13, 14, 14, 14, 14]), true);
        assert_eq!(is_four_of_a_kind(&vec![13, 13, 14, 14, 14]), false);
        assert_eq!(is_four_of_a_kind(&vec![13, 13, 13, 14, 14]), false);
        assert_eq!(is_four_of_a_kind(&vec![13, 13, 13, 13, 14]), true);
        assert_eq!(is_four_of_a_kind(&vec![13, 13, 13, 13, 13]), true);
        assert_eq!(is_four_of_a_kind(&vec![1, 13, 13, 13, 13]), true);
        assert_eq!(is_four_of_a_kind(&vec![1, 12, 12, 12, 13]), true);
        assert_eq!(is_four_of_a_kind(&vec![1, 12, 13, 13, 13]), true);
        assert_eq!(is_four_of_a_kind(&vec![1, 1, 12, 12, 13]), true);
        assert_eq!(is_four_of_a_kind(&vec![1, 1, 12, 13, 13]), true);
        assert_eq!(is_four_of_a_kind(&vec![1, 1, 1, 12, 13]), true);
    }

    #[test]
    fn is_full_house_test() {
        // Cards are sorted from lowest to highest

        // j, a, a, b, b
        // j, j, a, a, b
        // j, j, a, b, b
        // j, j, j, a, a
        // j, j, j, a, b
        assert_eq!(is_full_house(&vec![14, 14, 14, 14, 14]), false);
        assert_eq!(is_full_house(&vec![13, 14, 14, 14, 14]), false);
        assert_eq!(is_full_house(&vec![13, 13, 14, 14, 14]), true);
        assert_eq!(is_full_house(&vec![13, 13, 13, 14, 14]), true);
        assert_eq!(is_full_house(&vec![13, 13, 13, 13, 14]), false);
        assert_eq!(is_full_house(&vec![13, 13, 13, 13, 13]), false);
        assert_eq!(is_full_house(&vec![1, 13, 13, 14, 14]), true);
        assert_eq!(is_full_house(&vec![1, 1, 13, 13, 14]), true);
        assert_eq!(is_full_house(&vec![1, 1, 13, 14, 14]), true);
        assert_eq!(is_full_house(&vec![1, 1, 1, 13, 13]), false);
        assert_eq!(is_full_house(&vec![1, 1, 1, 13, 14]), false);
    }

    #[test]
    fn is_three_of_a_kind_test() {
        // Cards are sorted from lowest to highest
        assert_eq!(is_three_of_a_kind(&vec![14, 14, 14, 14, 14]), true);
        assert_eq!(is_three_of_a_kind(&vec![13, 14, 14, 14, 14]), true);
        assert_eq!(is_three_of_a_kind(&vec![13, 13, 14, 14, 14]), true);
        assert_eq!(is_three_of_a_kind(&vec![13, 13, 13, 14, 14]), true);
        assert_eq!(is_three_of_a_kind(&vec![13, 13, 13, 13, 14]), true);
        assert_eq!(is_three_of_a_kind(&vec![13, 13, 13, 13, 13]), true);
        assert_eq!(is_three_of_a_kind(&vec![12, 13, 13, 13, 14]), true);
        assert_eq!(is_three_of_a_kind(&vec![12, 13, 13, 14, 14]), false);
        assert_eq!(is_three_of_a_kind(&vec![12, 12, 13, 13, 14]), false);
        assert_eq!(is_three_of_a_kind(&vec![1, 12, 12, 13, 14]), true);
        assert_eq!(is_three_of_a_kind(&vec![1, 12, 13, 13, 14]), true);
        assert_eq!(is_three_of_a_kind(&vec![1, 12, 13, 14, 14]), true);
        assert_eq!(is_three_of_a_kind(&vec![1, 1, 13, 13, 14]), true);
        assert_eq!(is_three_of_a_kind(&vec![1, 1, 13, 14, 14]), true);
        assert_eq!(is_three_of_a_kind(&vec![1, 1, 1, 13, 14]), false);
    }

    #[test]
    fn is_two_pairs_test() {
        // Cards are sorted from lowest to highest
        assert_eq!(is_two_pairs(&vec![14, 14, 14, 14, 14]), true);
        assert_eq!(is_two_pairs(&vec![13, 14, 14, 14, 14]), true);
        assert_eq!(is_two_pairs(&vec![13, 13, 14, 14, 14]), true);
        assert_eq!(is_two_pairs(&vec![12, 12, 13, 13, 14]), true);
        assert_eq!(is_two_pairs(&vec![13, 13, 13, 13, 14]), true);
        assert_eq!(is_two_pairs(&vec![13, 13, 13, 13, 13]), true);
        assert_eq!(is_two_pairs(&vec![12, 13, 13, 13, 14]), false);
        assert_eq!(is_two_pairs(&vec![12, 13, 13, 14, 14]), true);
        assert_eq!(is_two_pairs(&vec![1, 13, 13, 14, 14]), true);
        assert_eq!(is_two_pairs(&vec![1, 12, 12, 13, 14]), true);
        assert_eq!(is_two_pairs(&vec![1, 12, 13, 13, 14]), true);
        assert_eq!(is_two_pairs(&vec![1, 12, 13, 14, 14]), true);
    }

    #[test]
    fn is_one_pair_test() {
        // Cards are sorted from lowest to highest
        assert_eq!(is_one_pair(&vec![14, 14, 14, 14, 14]), true);
        assert_eq!(is_one_pair(&vec![13, 14, 14, 14, 14]), true);
        assert_eq!(is_one_pair(&vec![13, 13, 14, 14, 14]), true);
        assert_eq!(is_one_pair(&vec![12, 12, 13, 13, 14]), true);
        assert_eq!(is_one_pair(&vec![13, 13, 13, 13, 14]), true);
        assert_eq!(is_one_pair(&vec![13, 13, 13, 13, 13]), true);
        assert_eq!(is_one_pair(&vec![12, 13, 13, 13, 14]), true);
        assert_eq!(is_one_pair(&vec![12, 13, 13, 14, 14]), true);
        assert_eq!(is_one_pair(&vec![12, 13, 14, 14, 14]), true);
        assert_eq!(is_one_pair(&vec![11, 12, 13, 13, 14]), true);
        assert_eq!(is_one_pair(&vec![11, 12, 12, 13, 14]), true);
        assert_eq!(is_one_pair(&vec![11, 11, 12, 13, 14]), true);
        assert_eq!(is_one_pair(&vec![11, 12, 13, 14, 14]), true);
        assert_eq!(is_one_pair(&vec![10, 11, 12, 13, 14]), false);
        assert_eq!(is_one_pair(&vec![1, 11, 12, 13, 14]), true);
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
        assert_eq!(hand_to_cards("32TJK"), vec![1, 2, 3, 10, 13]);
    }

    #[test]
    fn first_is_higher_test() {
        assert_eq!(first_is_higher("33332", "2AAAA"), true);
        assert_eq!(first_is_higher("2AAAA", "33332"), false);
        assert_eq!(first_is_higher("77888", "77788"), true);
        assert_eq!(first_is_higher("77788", "77888"), false);
        
        // From test set
        assert_eq!(first_is_higher("32T3K", "T55J5"), false);
        assert_eq!(first_is_higher("32T3K", "QQQJA"), false);
        assert_eq!(first_is_higher("32T3K", "KK677"), false);
        assert_eq!(first_is_higher("32T3K", "KTJJT"), false);

        assert_eq!(first_is_higher("T55J5", "32T3K"), true);
        assert_eq!(first_is_higher("T55J5", "QQQJA"), false);
        assert_eq!(first_is_higher("T55J5", "KK677"), true);
        assert_eq!(first_is_higher("T55J5", "KTJJT"), false);

        assert_eq!(first_is_higher("QQQJA", "32T3K"), true);
        assert_eq!(first_is_higher("QQQJA", "T55J5"), true);
        assert_eq!(first_is_higher("QQQJA", "KK677"), true);
        assert_eq!(first_is_higher("QQQJA", "KTJJT"), false);

        assert_eq!(first_is_higher("KK677", "32T3K"), true);
        assert_eq!(first_is_higher("KK677", "T55J5"), false);
        assert_eq!(first_is_higher("KK677", "QQQJA"), false);
        assert_eq!(first_is_higher("KK677", "KTJJT"), false);

        assert_eq!(first_is_higher("KTJJT", "32T3K"), true);
        assert_eq!(first_is_higher("KTJJT", "T55J5"), true);
        assert_eq!(first_is_higher("KTJJT", "QQQJA"), true);
        assert_eq!(first_is_higher("KTJJT", "KK677"), true);
    }

    #[test]
    fn first() {
        assert_eq!(first_is_higher("JJJJ2", "32T3K"), true);
        assert_eq!(first_is_higher("JJJJ2", "T55J5"), true);
        assert_eq!(first_is_higher("JJJJ2", "QQQJA"), true);
        assert_eq!(first_is_higher("JJJJ2", "KK677"), true);
        assert_eq!(first_is_higher("JJJJ2", "KTJJT"), true);
        assert_eq!(first_is_higher("JJJJJ", "22222"), false);
        assert_eq!(first_is_higher("JJJJJ", "KTJJT"), true);
    }

    #[test]
    fn extra_test_1() {
        let result = part2("JAAKK 1
JJJAK 2");
        assert_eq!(result, "5".to_string());
    }

    #[test]
    fn extra_test_2() {
        let result = part2("2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41");
        assert_eq!(result, "6839".to_string());
    }

    #[test]
    fn it_works2() {
        let result = part2(INPUT);
        assert_eq!(result, "5905".to_string());
    }
}