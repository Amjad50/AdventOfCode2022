use std::{env, fs, io};

fn main() -> io::Result<()> {
    let file_data = fs::read_to_string(env::args().nth(1).expect("Input file arg"))?;

    let mut sum = 0;
    let mut top3 = [0; 3];
    // add one empty line at the end
    file_data.lines().chain([""]).for_each(|line| {
        if line.is_empty() {
            if top3[0] <= sum {
                top3[2] = top3[1];
                top3[1] = top3[0];
                top3[0] = sum;
            } else if top3[1] <= sum {
                top3[2] = top3[1];
                top3[1] = sum;
            } else if top3[2] <= sum {
                top3[2] = sum;
            }
            sum = 0;
        } else {
            sum += line.parse::<u32>().unwrap();
        }
    });

    println!("Part1: max: {}", top3[0]);
    println!("Part2: top3: {:?}, sum: {}", top3, top3.iter().sum::<u32>());

    Ok(())
}
