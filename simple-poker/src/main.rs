use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq)]

enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];

    for suit in suits {
        for rank in 1..=13 {
            deck.push(Card { suit, rank });
        }
    }
    let mut rng = rand::rng();
    deck.shuffle(&mut rng);

    let mut hand: Vec<Card> = Vec::new();
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }

    hand.sort_by_key(|card| card.rank);

    println!("---Hand---");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}:Suit: {:?}, rank: {:}", i + 1, card.suit, card.rank);
    }

    println!("入れ替えたいカード番号を入力してください。（例：1,2,3）");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
    for number in numbers {
        hand[number - 1] = deck.pop().unwrap();
    }

    println!("---Hand---");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}:Suit: {:?}, rank: {:}", i + 1, card.suit, card.rank);
    }

    let suit = hand.first().unwrap().suit;
    let flash = hand.iter().all(|c| c.suit == suit);

    let mut counts = 0;
    for i in 0..hand.len() - 1 {
        for j in i + 1..hand.len() {
            if hand[i].rank == hand[j].rank {
                counts += 1;
            }
        }
    }

    if flash {
        println!("Flash!");
    } else if counts >= 1 {
        println!("{}ペア！", counts);
    } else {
        println!("役割なし...");
    }
}
