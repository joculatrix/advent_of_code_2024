use std::collections::HashMap;

/******************************************************************************
 *                                  PROBLEM 1                                 *
 ******************************************************************************/

pub fn prob5_1(input: &str) {
    let (rules, updates) = parse_input(input);
    let sum = validate_updates(rules, updates);

    println!("[5:1] Sum of valid middle pages: {}", sum);
}

struct PrintRules {
    // List of page numbers which should not come after this one.
    no_after: Vec<u8>,
}

// Outputs a map of page numbers (u8) mapped to PrintRules describing which
// pages shouldn't come after them, as well as a Vec<Vec<u8>> holding the
// print updates lists.
fn parse_input(input: &str) -> (HashMap<u8, PrintRules>, Vec<Vec<u8>>) {
    let mut rules: HashMap<u8, PrintRules> = HashMap::new();
    let mut updates = vec![];

    for line in input.lines() {
        if let Some(i) = line.find('|') {
            let (left, right) = line.split_at(i);

            let left = left.parse::<u8>().unwrap();
            let right = right[1..].parse::<u8>().unwrap();

            match rules.get_mut(&right) {
                Some(rules) => { rules.no_after.push(left); }
                None => {
                    rules.insert(right, PrintRules { no_after: vec![left] });
                }
            }
        } else {
            if line.is_empty() {
                continue;
            }

            let pages = line.split(',')
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();

            updates.push(pages);
        }
    }

    (rules, updates)
}

fn validate_updates(
    rules: HashMap<u8, PrintRules>,
    updates: Vec<Vec<u8>>
) -> u64 {
    let mut middle_page_sum = 0_u64;

    'outer: for update in updates {
        for i in 0..update.len() {
            let Some(rules) = rules.get(&update[i]) else {
                continue;
            };

            let no_after = &rules.no_after;

            for j in (i + 1)..update.len() {
                if no_after.contains(&update[j]) {
                    continue 'outer;
                }
            }
        }

        middle_page_sum += update[update.len() / 2] as u64;
    }

    middle_page_sum
}

/******************************************************************************
 *                                  PROBLEM 2                                 *
 ******************************************************************************/

pub fn prob5_2(input: &str) {
    let (rules, updates) = parse_input(input);
    let sum = validate_and_correct(rules, updates);

    println!("[5:2] Sum after corrections: {}", sum);
}

fn validate_and_correct(
    rules: HashMap<u8, PrintRules>,
    updates: Vec<Vec<u8>>
) -> u64 {
    let mut middle_page_sum = 0_u64;

    for mut update in updates {
        let mut first_loop = true;
        loop {
            let mut swapped = false;
            for i in 0..update.len() {
                let Some(rules) = rules.get(&update[i]) else {
                    continue;
                };
                let no_after = &rules.no_after;                
                for j in (i + 1)..update.len() {
                    if no_after.contains(&update[j]) {
                        update.swap(i, j);
                        swapped = true;
                    }
                }
            }

            // no swaps means the update is valid:
            if !swapped {
                break;
            }

            first_loop = false;
        }

        // we only want the sum from previously incorrect updates
        if !first_loop {
            middle_page_sum += update[update.len() / 2] as u64;
        }
    }

    middle_page_sum
}
