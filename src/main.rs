use rand::seq::SliceRandom;
use std::collections::HashMap;

fn main() {
    let mut deck_original: Vec<u32> = vec![];
    let mut number_zero = vec![0; 12];
    let mut number_one = vec![1; 4];
    let mut number_two = vec![2; 6];
    // let mut number_three = vec![3; 12];
    // let mut number_four = vec![4; 8];
    // let mut number_five = vec![5; 8];
    // let mut number_six = vec![6; 0];
    // let mut number_seven = vec![7; 0];
    // let mut number_eight = vec![8; 0];
    let mut other = vec![99; 22];

    deck_original.append(&mut number_zero);
    deck_original.append(&mut number_one);
    deck_original.append(&mut number_two);
    // deck_original.append(&mut number_three);
    // deck_original.append(&mut number_four);
    // deck_original.append(&mut number_five);
    // deck_original.append(&mut number_six);
    // deck_original.append(&mut number_seven);
    // deck_original.append(&mut number_eight);
    deck_original.append(&mut other);

    let energy_list: Vec<u32> = (0..11).collect();
    let mut hand_hash_original: HashMap<u32, u32> = HashMap::new();
    for e in energy_list.iter() {
        hand_hash_original.entry(*e).or_insert(0);
    }

    let iterate_number = 1_000_000;
    let mut movable = 0;

    for _ in 0..iterate_number {
        // println!("{}", "#".repeat(120));

        let mut deck = deck_original.clone();
        let mut rng = rand::thread_rng();
        deck.shuffle(&mut rng);

        // println!("deck length: {:?}", deck.len());
        // println!("deck: {:?}", deck);

        let hand = deck.split_off(deck.len() - 7);
        let mut hand_hash = hand_hash_original.clone();
        for h in hand.iter() {
            let counter = hand_hash.entry(*h).or_insert(0);
            *counter += 1;
        }

        // println!("{:?}", hand_hash);
        // println!("{:?}", hand_hash.get(&0).unwrap());

        if (*hand_hash.get(&0).unwrap() >= 3)
            || (*hand_hash.get(&0).unwrap() >= 2 && *hand_hash.get(&1).unwrap() >= 1)
            || (*hand_hash.get(&0).unwrap() >= 1 && *hand_hash.get(&1).unwrap() >= 2)
            || (*hand_hash.get(&0).unwrap() >= 1
                && *hand_hash.get(&1).unwrap() >= 1
                && *hand_hash.get(&2).unwrap() >= 1)
        // (*hand_hash.get(&0).unwrap() >= 2)
        //     || (*hand_hash.get(&0).unwrap() >= 1 && *hand_hash.get(&1).unwrap() >= 1)
        {
            movable += 1
        } else {
            let hand = deck.split_off(deck.len() - 7);
            let mut hand_hash = hand_hash_original.clone();
            for h in hand.iter() {
                let counter = hand_hash.entry(*h).or_insert(0);
                *counter += 1;
            }
            if (*hand_hash.get(&0).unwrap() >= 3)
                || (*hand_hash.get(&0).unwrap() >= 2 && *hand_hash.get(&1).unwrap() >= 1)
                || (*hand_hash.get(&0).unwrap() >= 1 && *hand_hash.get(&1).unwrap() >= 2)
                || (*hand_hash.get(&0).unwrap() >= 1
                    && *hand_hash.get(&1).unwrap() >= 1
                    && *hand_hash.get(&2).unwrap() >= 1)
            // (*hand_hash.get(&0).unwrap() >= 2)
            //     || (*hand_hash.get(&0).unwrap() >= 1 && *hand_hash.get(&1).unwrap() >= 1)
            {
                movable += 1
            }
        }

        // println!("hand: {:?}", hand);
        // println!("deck length: {:?}", deck.len());
        // println!("deck: {:?}", deck);
    }

    println!("movable hand count: {}/{}", movable, iterate_number);
    println!(
        "movable hand probability: {}%",
        (movable as f32 / iterate_number as f32) * 100f32
    );
}
