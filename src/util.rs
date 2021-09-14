use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

/// Returns a vector of String. The idea is to use this format to consume lines
/// from the files. We can also mock this out by passing Vec<String> to the solutions
/// since they expect this format as well.
pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn read_int_code_memory(filename: impl AsRef<Path>) -> Vec<i32> {
    lines_from_file(filename)
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>())
        .filter_map(Result::ok)
        .collect()
}

pub struct Permutations {
    counter: u32,
    current: Vec<u32>,
    limit: u32,
    n: usize,
    stack_frames: Vec<IterState>,
    stack_ptr: usize,
}

impl Permutations {
    pub fn new(n: usize) -> Self {
        let mut arr = vec![0u32; n];
        let mut stack_frames = Vec::with_capacity(n);
        let mut limit = 1u32;
        for (i, val) in arr.iter_mut().enumerate() {
            *val = i as u32;
            limit *= (i as u32) + 1;
            stack_frames.push(IterState::new(n - i));
        }
        Self {
            counter: 0,
            current: arr,
            limit,
            n,
            stack_frames,
            // Point to the top of the stack
            stack_ptr: 0,
        }
    }

    fn call_into_next_frame(&mut self) {
        self.stack_ptr += 1;
        self.stack_frames[self.stack_ptr as usize] =
            IterState::new((self.n - self.stack_ptr) as usize);
    }

    fn exit_current_frame(&mut self) {
        if self.stack_ptr >= 1 {
            self.stack_ptr -= 1;
        }
    }
}

struct IterState {
    i: isize,
    depth: usize,
    in_loop: bool,
}

impl IterState {
    fn new(depth: usize) -> Self {
        Self {
            depth,
            i: 0,
            in_loop: false,
        }
    }

    fn copy(&self) -> Self {
        Self {
            i: self.i,
            in_loop: self.in_loop,
            depth: self.depth,
        }
    }
}

impl Iterator for Permutations {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        // Once we have exceeded the number of possible permutations
        // return None
        if self.counter >= self.limit {
            return None;
        }

        // At this point the code is pretty much the way it would be. One way to clean this up is
        // to break up the steps into instructions and map them to numbers or something so when we
        // go through here we check the stack frame but also the instruction pointer within the frame
        // that is what instruction are we executing inside the function call.
        loop {
            let mut current_frame = self
                .stack_frames
                .get(self.stack_ptr as usize)
                .unwrap()
                .copy();

            if current_frame.depth == 1 {
                self.exit_current_frame();
                self.counter += 1;
                return Some(self.current.clone());
            }

            if !current_frame.in_loop {
                current_frame.in_loop = true;
                self.stack_frames[self.stack_ptr as usize] = current_frame;
                self.call_into_next_frame();
                continue;
            }

            // If we have completed the for loop we exit
            if current_frame.i >= (current_frame.depth - 1) as isize {
                self.exit_current_frame();
                continue;
            }

            // If we have no idea what i is because we haven't set the loop, we know to start the loop now
            if current_frame.i == -1 {
                current_frame.i = 0;
            }

            let swap_index = if current_frame.depth % 2 == 0 {
                current_frame.i as usize
            } else {
                0
            };
            self.current.swap(swap_index, current_frame.depth - 1);
            current_frame.i += 1;
            self.stack_frames[self.stack_ptr as usize] = current_frame;
            self.call_into_next_frame();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::Permutations;

    #[test]
    fn test_perm_iterator() {
        assert_eq!(Permutations::new(1).into_iter().count(), 1);
        assert_eq!(Permutations::new(2).into_iter().count(), 2);
        assert_eq!(Permutations::new(3).into_iter().count(), 6);
        assert_eq!(Permutations::new(4).into_iter().count(), 24);
        assert_eq!(Permutations::new(5).into_iter().count(), 120);
    }

    #[test]
    fn test_three_elem_permuration() {
        assert_eq!(
            vec![
                vec![0u32, 1u32, 2u32],
                vec![1u32, 0u32, 2u32],
                vec![2u32, 0u32, 1u32],
                vec![0u32, 2u32, 1u32],
                vec![1u32, 2u32, 0u32],
                vec![2u32, 1u32, 0u32],
            ],
            Permutations::new(3).into_iter().collect::<Vec<Vec<u32>>>()
        );
    }
}
