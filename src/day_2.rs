/******************************************************************************
 *                                 PROBLEM 1                                  *
 ******************************************************************************/

pub fn prob2_1(input: &str) {
    let reports = parse_reports(input);
    let safe_count = check_safety(&reports, false);

    println!("[2:1] Safe reports: {}", safe_count);
}

fn parse_reports(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|line|
            line.split_whitespace()
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        )
        .collect::<Vec<Vec<u8>>>()
}

fn check_safety(reports: &Vec<Vec<u8>>, dampened: bool) -> usize {
    let mut count = 0;
    let mut unsafe_ = vec![];

    'outer: for (index, report) in reports.iter().enumerate() {
        let mut ascending = None;
        let mut prev = None;

        for level in report {
            match prev {
                None => (),
                Some(prev) => match ascending {
                    None => {
                        if !(1..=3).contains(&u8::abs_diff(*level, prev)) {
                            if dampened { unsafe_.push(index); }
                            continue 'outer;
                        }
                        ascending = Some(*level > prev);
                    },
                    Some(ascending) => {
                        if (ascending && *level < prev)
                        || (!ascending && *level > prev)
                        || !(1..=3).contains(&u8::abs_diff(*level, prev)) {
                            if dampened { unsafe_.push(index); }
                            continue 'outer;
                        }
                    }
                }
            }
            prev = Some(*level);
        }

        count += 1;
    }
    
    if dampened {
        count = check_recoverable(reports, unsafe_, count);
    }

    count
}

/******************************************************************************
 *                                 PROBLEM 2                                  *
 ******************************************************************************/

pub fn prob2_2(input: &str) {
    let reports = parse_reports(input);
    let safe_count = check_safety(&reports, true);

    println!("[2:2] Safe reports: {}", safe_count);
}

fn check_recoverable(
    reports: &Vec<Vec<u8>>,
    unsafe_: Vec<usize>,
    mut count: usize,
) -> usize {
    for i in unsafe_ {
        for j in 0..reports[i].len() {
            let mut report = reports[i].clone();
            report.remove(j);

            let safe = check_safety(&vec![report], false) == 1;

            if safe {
                count += 1;
                break;
            }
        }
    }

    count
}
