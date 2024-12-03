use std::sync::LazyLock;
use regex::Regex;
use tailcall::tailcall;

/******************************************************************************
 *                                  PROBLEM 1                                 *
 ******************************************************************************/

pub fn prob3_1(input: &str) {
    let sum = parse_memory(input, 0);

    println!("[3:1] Sum of valid multiplications: {}", sum);
}

#[tailcall]
fn parse_memory(input: &str, sum: u64) -> u64 {
    match input.find("mul(") {
        None => sum,
        Some(i) => {
            if input.len() < i + 4 {
                return sum;
            }

            let Some(mut j) = input[i + 4..].find(',') else { return sum; };
            j += i + 4;

            let Some(mut k) = input[j..].find(')') else { return sum; };
            k += j;

            let x = match input[i + 4 .. j].parse::<u64>() {
                Err(_) => { return parse_memory(&input[i + 4..], sum); }
                Ok(x) => x,
            };

            let y = match input[j + 1 .. k].parse::<u64>() {
                Err(_) => { return parse_memory(&input[i + 4..], sum); }
                Ok(y) => y,
            };

            parse_memory(&input[k..], sum + (x * y))
        }
    }
}

/******************************************************************************
 *                                  PROBLEM 2                                 *
 ******************************************************************************/
static DO: LazyLock<Regex> = LazyLock::new(||
    Regex::new(r"\Ado\(\)(?<tail>(?s).*)").unwrap()
);

static DONT: LazyLock<Regex> = LazyLock::new(||
    Regex::new(r"\Adon't\(\)(?<tail>(?s).*)").unwrap()
);

static MUL: LazyLock<Regex> = LazyLock::new(||
    Regex::new(r"\Amul\((?<left>[0-9]+),(?<right>[0-9]+)\)(?<tail>(?s).*)").unwrap()
);

pub fn prob3_2(input: &str) {
    let sum = parse_memory_conditional(input);

    println!("[3:2] Sum of valid, enabled multiplications: {}", sum);
}

fn parse_memory_conditional(input: &str) -> u64 {
    #[tailcall]
    fn inner(input: &str, sum: u64, enabled: bool) -> u64 {
        if 16 <= input.len() { println!("\"{}\"...", &input[..16]); }
        if let Some(capture) = (&*DO).captures(&input) {
            println!("DO");
            inner(capture.name("tail").unwrap().as_str(), sum, true)
        } else if let Some(capture) = (&*DONT).captures(&input) {
            println!("DONT");
            inner(capture.name("tail").unwrap().as_str(), sum, false)
        } else if let Some(capture) = (&*MUL).captures(&input) {
            println!("MUL {} {}", &capture["left"], &capture["right"]);
            inner(
                capture.name("tail").unwrap().as_str(),
                if enabled {
                    sum + (
                        &capture["left"].parse::<u64>().unwrap()
                        * &capture["right"].parse::<u64>().unwrap()
                    )
                } else {
                    sum
                },
                enabled,
            )
        } else {
            println!("NONE");
            if input.is_empty() {
                sum
            } else {
                inner(&input[1..], sum, enabled)
            }
        }
    }

    inner(input, 0, true)
}
