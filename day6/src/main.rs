use std::{env, fs, io};

struct PackerFinder<const N: usize> {
    last_n_chars: [char; N],
    require_skip: usize,
    i: usize,
}

impl<const N: usize> PackerFinder<N> {
    pub fn next_inp(&mut self, c: char) -> bool {
        let mut new_char = true;
        for j in 0..(N - 1) {
            self.last_n_chars[j] = self.last_n_chars[j + 1];
            if c == self.last_n_chars[j] {
                new_char = false;
                let empty_space = (N - 1).saturating_sub(self.i);
                self.require_skip = (self.require_skip.saturating_sub(1)).max(j - empty_space);
            }
        }
        self.last_n_chars[N - 1] = c;
        if new_char && self.i > N - 1 {
            if self.require_skip > 0 {
                self.require_skip -= 1;
            } else {
                self.i += 1;
                return true;
            }
        }
        self.i += 1;
        false
    }
}

fn main() -> io::Result<()> {
    // this time its only 1 line input, no need to split by newline
    let input = fs::read_to_string(env::args().nth(1).expect("Input file arg"))?;
    let input = input.trim_end();

    let mut p1_packer_finder = PackerFinder::<4> {
        last_n_chars: [' '; 4],
        require_skip: 0,
        i: 0,
    };
    let mut p2_message_finder = PackerFinder::<14> {
        last_n_chars: [' '; 14],
        require_skip: 0,
        i: 0,
    };

    let mut p1_start_of_packet = 0;
    let mut p2_start_of_message = 0;

    let mut chars_iter = input.chars().enumerate();

    let mut p1_done = false;
    while let Some((i, c)) = chars_iter.next() {
        if !p1_done {
            if p1_packer_finder.next_inp(c) {
                p1_start_of_packet = i + 1;
                p1_done = true;
            }
        }
        if p2_message_finder.next_inp(c) {
            p2_start_of_message = i + 1;
            break;
        }
    }

    println!("Part 1: packet starts at: {}", p1_start_of_packet);
    println!("Part 2: message starts at: {}", p2_start_of_message);

    Ok(())
}
