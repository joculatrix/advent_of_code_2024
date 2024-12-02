use std::collections::HashMap;

/******************************************************************************
 *                                 PROBLEM 1                                  *
 ******************************************************************************/

pub fn prob1_1(input: &str) {
    let (list_1, list_2) = parse_pairs(input);
    let distance = list_distance(list_1, list_2);

    println!("[1:1] Total distance between lists: {}", distance);
}

fn parse_pairs(input: &str) -> (Vec<u64>, Vec<u64>) {
    let (mut list_1, mut list_2) = (vec![], vec![]);

    for line in input.lines() {
        let mut pair = line.split_whitespace().map(|s| s.parse::<u64>().unwrap());
        let a = pair.next().expect("expected well-formed input");
        let b = pair.next().expect("expected well-formed input");
        
        // ensure only 2 numbers were taken
        if pair.next().is_some() { panic!() }

        list_1.insert(
            match list_1.binary_search(&a) { Ok(i) => i, Err(i) => i },
            a,
        );
        list_2.insert(
            match list_2.binary_search(&b) { Ok(i) => i, Err(i) => i },
            b,
        );
    }

    (list_1, list_2)
}

fn list_distance(list_1: Vec<u64>, list_2: Vec<u64>) -> u64 {
    list_1.iter().zip(list_2.iter())
        .fold(0_u64, |distance, (a, b)|
            distance + u64::abs_diff(*a, *b)
        )
}

/******************************************************************************
 *                                 PROBLEM 2                                  *
 ******************************************************************************/

pub fn prob1_2(input: &str) {
    let map = parse_for_similarity(input);
    let score = calculate_similarity(map);

    println!("[1:2] Similarity score: {}", score);
}

fn parse_for_similarity(input: &str) -> HashMap<u64, (u32, u32)> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let mut pair = line.split_whitespace().map(|s| s.parse::<u64>().unwrap());
        let a = pair.next().expect("expected well-formed input");
        let b = pair.next().expect("expected well-formed input");
        
        // ensure only 2 numbers were taken
        if pair.next().is_some() { panic!() }

        insert_left(&mut map, a);
        insert_right(&mut map, b);
    }

    map
}

fn insert_left(map: &mut HashMap<u64, (u32, u32)>, x: u64) {
    match map.get_mut(&x) {
        Some((left, _right)) => { *left += 1; },
        None => { map.insert(x, (1, 0)); },
    }
}

fn insert_right(map: &mut HashMap<u64, (u32, u32)>, x: u64) {
    match map.get_mut(&x) {
        Some((_left, right)) => { *right += 1; },
        None => { map.insert(x, (0, 1)); },
    }
}

fn calculate_similarity(map: HashMap<u64, (u32, u32)>) -> u64 {
    map.iter()
        .fold(0_u64, |score, (value, (left, right))|
            score + (value * (*left as u64) * (*right as u64))
        )
}
