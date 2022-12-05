use std::{env, fs, io};

fn main() -> io::Result<()> {
    let file_data = fs::read_to_string(env::args().nth(1).expect("Input file arg"))?;

    let mut p1_stacks = Vec::<Vec<char>>::new();

    let lines = file_data.lines();
    let mut stacks_input_len = 0;
    lines
        .clone()
        .take_while(|l| l.chars().nth(1) != Some('1'))
        .for_each(|line| {
            stacks_input_len += 1;
            let mut counter = 0;
            let mut i = 0;
            for c in line.chars() {
                counter = (counter + 1) % 4;
                if counter == 2 && c != ' ' {
                    if p1_stacks.len() <= i {
                        p1_stacks.resize(i + 1, Vec::new());
                    }
                    p1_stacks[i].insert(0, c);
                } else if counter == 3 {
                    i += 1;
                }
            }
        });

    // copy into p2 stack (since the order of moving will be different)
    let mut p2_stacks = p1_stacks.clone();

    lines.skip(stacks_input_len + 2).for_each(|line| {
        let mut l = line.split_whitespace();
        // take every other word
        let count = l.nth(1).unwrap().parse::<usize>().unwrap();
        let from = l.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = l.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        // part 1
        let stack_len = p1_stacks[from].len();
        let stack_drain = p1_stacks[from]
            .drain(stack_len - count..)
            .rev()
            .collect::<Vec<_>>();

        p1_stacks[to].extend(stack_drain);

        // part 2
        let stack_len = p2_stacks[from].len();
        let stack_drain = p2_stacks[from]
            .drain(stack_len - count..)
            .collect::<Vec<_>>();

        p2_stacks[to].extend(stack_drain);
    });

    let p1 = p1_stacks
        .iter()
        .filter_map(|s| s.last().cloned())
        .collect::<String>();
    let p2 = p2_stacks
        .iter()
        .filter_map(|s| s.last().cloned())
        .collect::<String>();
    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", p2);

    Ok(())
}
