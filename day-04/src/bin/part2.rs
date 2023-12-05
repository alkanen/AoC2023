use std::cmp;

fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn get_original_cards(input: &str) -> Vec<(usize, usize)> {
    let mut cards = Vec::new();
    let mut ctr = 0;
    for line in input.lines() {
        let (card_part, rounds_part) = line.split_once(": ").unwrap();
        let num_vec = card_part.split_whitespace().collect::<Vec<&str>>();
        let num = num_vec[1];

        let (winning_numbers_part, own_numbers_part) = rounds_part.split_once(" | ").unwrap();
        let winning_numbers = winning_numbers_part.split_whitespace().collect::<Vec<&str>>();
        let own_numbers = own_numbers_part.split_whitespace().collect::<Vec<&str>>();

        let mut score = 0;
        for winning_number in winning_numbers.iter() {
            if own_numbers.contains(winning_number) {
                score += 1;
            }
        }

        //println!("{} Card '{}': {} (score {})", ctr, num, score, score);
        ctr += 1;
        cards.push((num.parse::<usize>().unwrap(), score));
    }

    return cards
}

fn generate_card_result_list(cards: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    // let cards = get_original_cards(input);

    let mut result_list = Vec::new();
    //println!("Cards: {:?}", cards);
    for (i, card) in cards.iter().enumerate() {
        let (_num, score) = card;
        let mut result = Vec::new();
        for j in i+1..cmp::min(i + score + 1, cards.len()) {
            result.push(cards[j].0);
        }
        result_list.push(result);
    }

    return result_list;
}

fn find_new_cards(cards: &Vec<usize>, result_list: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut new_cards = Vec::new();

    for card in cards.iter() {
        let ncs = result_list[*card - 1].clone();

        for nc in ncs.iter() {
            new_cards.push(*nc);
        }
    }

    return new_cards;
}

fn part2(input: &str) -> String {
    let mut all_cards : Vec<usize> = Vec::new();
    let cards = get_original_cards(input);
    let rl = generate_card_result_list(&cards);

    let mut old_cards = Vec::new();
    for card in cards.iter() {
        let (num, _) = card;
        old_cards.push(*num);
    }

    loop {
        if old_cards.len() == 0 {
            break;
        }

        //println!("Old cards: {:?}", old_cards);
        for card in old_cards.iter() {
            all_cards.push(*card);
            //println!("  Card {}", card);
        }

        let new_cards = find_new_cards(&old_cards, &rl);
        old_cards = new_cards.clone();
    }

    return all_cards.len().to_string();
}

    /*
    let mut all_cards : Vec<usize> = Vec::new();

    let orig_cards = get_original_cards(input);
    let rl = generate_card_result_list(orig_cards);

    for card in orig_cards.iter() {
        let (num, _) = card;
        all_cards.push(*num);
    }


    return all_cards.len().to_string();
}
*/

/*
fn part2_old(input: &str) -> String {
    let mut sum = 0;
    let mut cards = Vec::new();
    for line in input.lines() {
        let (card_part, rounds_part) = line.split_once(": ").unwrap();
        let (_, num) = card_part.split_once(" ").unwrap();

        let (winning_numbers_part, own_numbers_part) = rounds_part.split_once(" | ").unwrap();
        let winning_numbers = winning_numbers_part.split_whitespace().collect::<Vec<&str>>();
        let own_numbers = own_numbers_part.split_whitespace().collect::<Vec<&str>>();

        let mut score = 0;
        for winning_number in winning_numbers.iter() {
            if own_numbers.contains(winning_number) {
                score += 1;
            }
        }

        cards.push((num.parse::<usize>().unwrap(), score));
    }


    let mut ctr = 1;
    let mut old_cards = cards;
    loop {
        if old_cards.len() == 0 {
            break;
        }

        let mut new_cards : Vec<(usize, usize)> = Vec::new();

        println!("Iteration {}: ", ctr);
        ctr += 1;
        for (i, card) in old_cards.iter().enumerate() {
            let (num, score) = card;
            println!("Card {}: {} (pos {})", num, score, i);

            sum += 1;

            for j in i+1..cmp::min(i + score + 1, old_cards.len()) {
                println!("  Add card {}: {} (pos {})", old_cards[j].0, old_cards[j].1, j);
                new_cards.push(old_cards[j]);
            }
        }

        println!("");
        
        old_cards = new_cards.clone();
    }

    return sum.to_string();
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn get_original_cards_test() {
        println!("Original cards: ");
        let cards = get_original_cards(INPUT);
        for (i, card) in cards.iter().enumerate() {
            println!("Card {} -> {:?}", i, card);
        }
        assert_eq!(cards.len(), 6);
        assert_eq!(cards[0], (1, 4));
        assert_eq!(cards[1], (2, 2));
        assert_eq!(cards[2], (3, 2));
        assert_eq!(cards[3], (4, 1));
        assert_eq!(cards[4], (5, 0));
        assert_eq!(cards[5], (6, 0));
    }

    #[test]
    fn result_list_test() {
        let cards = get_original_cards(INPUT);

        println!("Result list: ");
        let rl = generate_card_result_list(&cards);
        for (i, card) in rl.iter().enumerate() {
            println!("Card {} -> {:?}", i, card);
        }
        assert_eq!(rl.len(), 6);
        assert_eq!(rl[0], vec![2, 3, 4, 5]);
        assert_eq!(rl[1], vec![3, 4]);
        assert_eq!(rl[2], vec![4, 5]);
        assert_eq!(rl[3], vec![5]);
        assert_eq!(rl[4], vec![]);
        assert_eq!(rl[5], vec![]);
    }

    #[test]
    fn find_new_cards_test() {
        let cards = get_original_cards(INPUT);
        let rl = generate_card_result_list(&cards);

        let mut old_cards = Vec::new();
        for card in cards.iter() {
            let (num, _) = card;
            old_cards.push(*num);
        }

        println!("New cards: ");
        let new_cards = find_new_cards(&old_cards, &rl);
        for (i, card) in new_cards.iter().enumerate() {
            println!("Card {} -> {:?}", i, card);
        }
        assert_eq!(new_cards.len(), 9);
        assert_eq!(new_cards[0], 2);
        assert_eq!(new_cards[1], 3);
        assert_eq!(new_cards[2], 4);
        assert_eq!(new_cards[3], 5);
        assert_eq!(new_cards[4], 3);
        assert_eq!(new_cards[5], 4);
        assert_eq!(new_cards[6], 4);
        assert_eq!(new_cards[7], 5);
        assert_eq!(new_cards[8], 5);
    }

    #[test]
    fn it_works2() {
        let result = part2(INPUT);
        assert_eq!(result, "30".to_string());
    }
}