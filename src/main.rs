use std::fs::read_to_string;

mod day_1;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    day: usize,
    problem: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (day, problem) = {
        use clap::Parser;
        
        let args = Args::parse();

        (args.day, args.problem)
    };

    match (day, problem) {
        (1, 1) => { day_1::prob1_1(&read_to_string("input/1.txt")?); },
        (1, 2) => { day_1::prob1_2(&read_to_string("input/1.txt")?); },
        _ => { println!("Invalid day and problem received. Exiting..."); }
    }

    Ok(())
}
