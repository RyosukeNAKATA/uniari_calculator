use rand::seq::SliceRandom;
use std::collections::HashMap;

fn main() {
    let mut deck_original: Vec<u32> = vec![];

    let zero = 12;
    let one = 4;
    let one_two = 0;
    let two = 6;
    let three = 0;
    let four = 0;
    let five = 0;
    let six = 0;
    let seven = 0;
    let eight = 0;
    let other = 50 - (zero + one + one_two + two + three + four);

    let mut number_zero = vec![0; zero];
    let mut number_one = vec![1; one];
    let mut number_one_two = vec![11; one_two];
    let mut number_two = vec![2; two];
    let mut number_three = vec![3; three];
    let mut number_four = vec![4; four];
    let mut number_five = vec![5; five];
    let mut number_six = vec![6; six];
    let mut number_seven = vec![7; seven];
    let mut number_eight = vec![8; eight];
    let mut number_other = vec![13; other];

    deck_original.append(&mut number_zero);
    deck_original.append(&mut number_one);
    // deck_original.append(&mut number_one_two);
    deck_original.append(&mut number_two);
    // deck_original.append(&mut number_three);
    // deck_original.append(&mut number_four);
    // deck_original.append(&mut number_five);
    // deck_original.append(&mut number_six);
    // deck_original.append(&mut number_seven);
    // deck_original.append(&mut number_eight);
    deck_original.append(&mut number_other);

    let energy_list: Vec<u32> = (0..15).collect();
    let mut hand_hash_original: HashMap<u32, u32> = HashMap::new();
    for e in energy_list.iter() {
        hand_hash_original.entry(*e).or_insert(0);
    }

    let iterate_number = 1_000_000;
    let mut movable = 0;

    for _ in 0..iterate_number {
        let mut deck = deck_original.clone();
        let mut rng = rand::thread_rng();
        deck.shuffle(&mut rng);

        let hand = deck.split_off(deck.len() - 7);
        let mut hand_hash = hand_hash_original.clone();
        for h in hand.iter() {
            let counter = hand_hash.entry(*h).or_insert(0);
            *counter += 1;
        }

        if (*hand_hash.get(&0).unwrap() >= 3) // 000
            || (*hand_hash.get(&0).unwrap() >= 2 && *hand_hash.get(&1).unwrap() >= 1) // 001
            || (*hand_hash.get(&0).unwrap() >= 1 && *hand_hash.get(&1).unwrap() >= 2) // 011
            || (*hand_hash.get(&0).unwrap() >= 2 && *hand_hash.get(&2).unwrap() >= 1) // 002
            || (*hand_hash.get(&0).unwrap() >= 1 
                && *hand_hash.get(&1).unwrap() >= 1
                && *hand_hash.get(&2).unwrap() >= 1) // 012
            || (*hand_hash.get(&0).unwrap() >= 1 && *hand_hash.get(&11).unwrap() >= 1) // 01(2)
        {
            movable += 1
        } else {
            let hand = deck.split_off(deck.len() - 7);
            let mut hand_hash = hand_hash_original.clone();
            for h in hand.iter() {
                let counter = hand_hash.entry(*h).or_insert(0);
                *counter += 1;
            }
            if (*hand_hash.get(&0).unwrap() >= 3) // 000
                || (*hand_hash.get(&0).unwrap() >= 2 && *hand_hash.get(&1).unwrap() >= 1) // 001
                || (*hand_hash.get(&0).unwrap() >= 1 && *hand_hash.get(&1).unwrap() >= 2) // 011
                || (*hand_hash.get(&0).unwrap() >= 2 && *hand_hash.get(&2).unwrap() >= 1) // 002
                || (*hand_hash.get(&0).unwrap() >= 1
                    && *hand_hash.get(&1).unwrap() >= 1
                    && *hand_hash.get(&2).unwrap() >= 1) // 012
                || (*hand_hash.get(&0).unwrap() >= 1 && *hand_hash.get(&11).unwrap() >= 1) // 01(2)
            {
                movable += 1
            }
        }
    }

    println!("movable hand count: {}/{}", movable, iterate_number);
    println!(
        "movable hand probability: {}%",
        (movable as f32 / iterate_number as f32) * 100f32
    );
}
