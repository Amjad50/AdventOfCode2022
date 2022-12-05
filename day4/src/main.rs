use std::{env, fs, io};

fn main() -> io::Result<()> {
    let file_data = fs::read_to_string(env::args().nth(1).expect("Input file arg"))?;

    let mut p1_sum = 0;
    let mut p2_sum = 0;

    file_data.lines().for_each(|line| {
        let (p1, p2) = line.split_once(',').unwrap();
        let (p11, p12) = p1.split_once('-').unwrap();
        let (p21, p22) = p2.split_once('-').unwrap();
        let p11 = p11.parse::<u32>().unwrap();
        let p12 = p12.parse::<u32>().unwrap();
        let p21 = p21.parse::<u32>().unwrap();
        let p22 = p22.parse::<u32>().unwrap();

        if (p11 >= p21 && p11 <= p22) || (p21 >= p11 && p21 <= p12) {
            p2_sum += 1;
            if (p11 >= p21 && p12 <= p22) || (p21 >= p11 && p22 <= p12) {
                p1_sum += 1;
            }
        }
    });

    println!("Part 1: {}", p1_sum);
    println!("Part 2: {}", p2_sum);

    Ok(())
}
