use std::{env, fs, io};

#[inline(always)]
fn get_priority_num(c: char) -> usize {
    if c.is_ascii_lowercase() {
        c as usize - 'a' as usize
    } else {
        c as usize - 'A' as usize + 26
    }
}

fn main() -> io::Result<()> {
    let file_data = fs::read_to_string(env::args().nth(1).expect("Input file arg"))?;

    let mut sum_priorities_p1 = 0;
    let mut sum_priorities_p2 = 0;

    let mut group_type_nums = [0; 26 * 2];

    file_data.lines().for_each(|line| {
        // a-z: 0-25
        // A-Z: 26-51
        // true if the type is found already (fixed set)
        let mut types_found = [false; 26 * 2];
        let (comp_1, comp_2) = line.split_at(line.len() / 2);

        for c in comp_1.chars() {
            let priority_num = get_priority_num(c);
            types_found[priority_num] = true;
        }
        // copy the current `types_found` to be used to check for compartment 1
        // content
        let c1_types_found = types_found;
        // stop adding to the priority sum if we already did this turn,
        // we can't `break` the loop because we need to look through all the items
        let mut p1_done = false;
        for c in comp_2.chars() {
            let priority_num = get_priority_num(c);
            if !p1_done && c1_types_found[priority_num] {
                sum_priorities_p1 += priority_num + 1; // index offset
                p1_done = true;
            }
            types_found[priority_num] = true;
        }
        for i in 0..group_type_nums.len() {
            // add 1 to the priority sum if the type is found anywhere
            group_type_nums[i] += types_found[i] as usize;
            // only one item is found 3 times, reset
            if group_type_nums[i] == 3 {
                sum_priorities_p2 += i + 1; // index offset

                group_type_nums = [0; 26 * 2];
                break;
            }
        }
    });

    println!("Part1: priority sum: {}", sum_priorities_p1);
    println!("Part2: priority sum: {}", sum_priorities_p2);

    Ok(())
}
