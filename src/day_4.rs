/******************************************************************************
 *                                  PROBLEM 1                                 *
 ******************************************************************************/

pub fn prob4_1(input: &str) {
    let chars = parse_word_search(input);
    let count = count_xmas(chars);

    println!("[4:1] 'XMAS' count: {}", count);
}

fn parse_word_search(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn count_xmas(input: Vec<Vec<char>>) -> usize {
    let mut count = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            // horizontal matches
            if j + 3 < input[i].len() {
                if matches!(
                    &input[i][j..=j + 3],
                    ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X']
                ) {
                    count += 1;
                }
            }
            // vertical matches
            if i + 3 < input.len() {
                if matches!(
                    &[input[i][j], input[i+1][j], input[i+2][j], input[i+3][j]],
                    ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X']
                ) {
                    count += 1;
                }
            }
            // down-right diagonal matches
            if j + 3 < input[i].len() && i + 3 < input.len() {
                if matches!(
                    &[input[i][j], input[i+1][j+1], input[i+2][j+2], input[i+3][j+3]],
                    ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X']
                ) {
                    count += 1;
                }
            }
            // down-left diagonal matches
            if j >= 3 && i + 3 < input.len() {
                if matches!(
                    &[input[i][j], input[i+1][j-1], input[i+2][j-2], input[i+3][j-3]],
                    ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X']
                ) {
                    count += 1;
                }
            }
        }
    }

    count
}

/******************************************************************************
 *                                  PROBLEM 2                                 *
 ******************************************************************************/

pub fn prob4_2(input: &str) {
    let chars = parse_word_search(input);
    let count = count_x_mas(chars);

    println!("[4:2] X-'MAS' count: {}", count);
}

fn count_x_mas(input: Vec<Vec<char>>) -> usize {
    let mut count = 0;

    for i in 0..input.len() - 2 {
        for j in 0..input[i].len() - 2 {
            if matches!(
                &[input[i][j], input[i+1][j+1], input[i+2][j+2]],
                ['M', 'A', 'S'] | ['S', 'A', 'M']
            ) && matches!(
                &[input[i][j+2], input[i+1][j+1], input[i+2][j]],
                ['M', 'A', 'S'] | ['S', 'A', 'M']
            ) {
                count += 1;
            }
        }
    }

    count
}
