use core::num;

use rand::{random, seq::SliceRandom};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    let mut deck = Vec::new();

    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    for suit in suits {
        for rank in 1..=13 {
            deck.push(Card {suit, rank });
        }
    }

    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    let mut hand = Vec::new();

    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }

    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    for (i, card) in hand.iter().enumerate() {
        println!("{}: {:?} {}", i+1, card.suit, card.rank);
    }

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();


    for n in numbers {
        hand[n-1] = deck.pop().unwrap();
    }

    for (i, card) in hand.iter().enumerate() {
        println!("{}: {:?} {}", i+1, card.suit, card.rank);
    }

    let suit = hand.first().unwrap().suit;
    let flash = hand.iter().all(|c| c.suit == suit);

    let mut count = 0;

    for i in 0..hand.len()-1 {
        for j in i+1..hand.len() {
            if hand[i].rank == hand[j].rank {
                count += 1;
            }
        }
    }

    if flash {
        println!("Flash");
    } else if count >= 3 {
        println!("Three card");
    } else if count >= 2 {
        println!("Two Pair");
    } else if count >= 1 {
        println!("One Pair");
    } else {
        println!("No");
    }
}
