use crate::utils::*;

type Card = Vec<Vec<i32>>;
type Cards = Vec<Card>;

pub fn q1() {
    let content = file_to_string("src/inputs/day4.txt");
    let mut parsed = parse_by_empty_lines(&content);
    let numbers: Vec<i32> = parsed[0].split(",").map(|x| parse_int(x)).collect();
    parsed.drain(0..1);

    let boards: Vec<Vec<&str>> = parsed.iter().map(|x| content_to_lines(x)).collect();

    let mut cards: Cards = boards
        .iter()
        .map(|b| trim_vec(b.to_vec()))
        .map(|x| split_whites(x))
        .collect();

    for n in numbers.iter() {
        cards = cards
            .into_iter()
            .map(|card| update_card(card.to_vec(), *n))
            .collect();

        let completed_cards: Cards = get_completed_cards(cards.clone());

        if completed_cards.len() == 1 {
            let total = completed_cards[0].iter().fold(0, |acc_row, row| {
                acc_row + row.iter().filter(|&&x| x > 0).fold(0, |acc, y| acc + y)
            });
            println!("D4Q1 {}", total * n);
            break;
        }
    }
}

fn get_completed_cards(cards: Cards) -> Cards {
    let completed = cards
        .into_iter()
        .filter(|card| is_card_complete(card))
        .collect();
    completed
}

pub fn q2() {
    let content = file_to_string("src/inputs/day4.txt");
    let mut parsed = parse_by_empty_lines(&content);
    let numbers: Vec<i32> = parsed[0].split(",").map(|x| parse_int(x)).collect();
    parsed.drain(0..1);

    let boards: Vec<Vec<&str>> = parsed.iter().map(|x| content_to_lines(x)).collect();

    let mut cards: Cards = boards
        .iter()
        .map(|b| trim_vec(b.to_vec()))
        .map(|x| split_whites(x))
        .collect();

    for n in numbers.iter() {
        cards = cards
            .iter()
            .map(|card| update_card(card.to_vec(), *n))
            .collect();
        let cards_backup = cards.clone();

        cards = cards
            .into_iter()
            .filter(|card| !is_card_complete(card))
            .collect();

        if cards.len() == 0 && is_card_complete(&cards_backup[0]) {
            let total = cards_backup[0].iter().fold(0, |acc_row, row| {
                acc_row + row.iter().filter(|&&x| x > 0).fold(0, |acc, y| acc + y)
            });
            println!("D4Q2 {}", total * n);
            break;
        }
    }
}

fn update_card_row(row: Vec<i32>, n: i32) -> Vec<i32> {
    let newrow = row
        .iter()
        .map(|&el| {
            if el == n {
                return -1;
            }
            return el;
        })
        .collect();

    newrow
}

fn update_card(card: Card, n: i32) -> Card {
    let newcard = card
        .iter()
        .map(|row| update_card_row(row.to_vec(), n))
        .collect();
    newcard
}

pub fn is_card_complete(card: &Card) -> bool {
    for row in card.iter() {
        let total = row.iter().fold(0, |acc, x| x + acc);
        if total == -5 {
            return true;
        }
    }

    // check columns
    for col in 0..5 {
        let mut total = 0;
        for row in 0..5 {
            total += card[row][col];
            if total == -5 {
                return true;
            }
        }
    }

    false
}

fn trim_vec(v: Vec<&str>) -> Vec<&str> {
    return v.iter().map(|x| x.trim()).collect();
}

fn split_whites(v: Vec<&str>) -> Card {
    return v
        .iter()
        .map(|x: &&str| x.split_whitespace().map(|x| parse_int(x)).collect())
        .collect();
}
