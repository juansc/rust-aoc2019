use std::path::Iter;
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
    stack_frames: Vec<IterState>,
    stack_ptr: u32,
}

impl Permutations {
    pub fn new(n: usize) -> Self {
        let mut arr = vec![0u32; n];
        let mut stack_frames = Vec::with_capacity(n);
        let mut limit = 1u32;
        for i in 0..n {
            arr[i] = i as u32;
            limit *= (i as u32) + 1;
            stack_frames.push(IterState::new(n - i));
        }
        Self {
            counter: 0,
            current: arr,
            limit,
            stack_frames,
            // Point to the top of the stack
            stack_ptr: 0,
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
            let f = self.stack_frames.get(self.stack_ptr as usize).unwrap();
            let mut current_frame = f.copy();

            if current_frame.depth == 1 {
                self.stack_frames[self.stack_ptr as usize] = IterState::new(current_frame.depth);
                if self.stack_ptr >= 1 {
                    self.stack_ptr -= 1;
                }
                self.counter += 1;
                return Some(self.current.clone());
            }

            if !current_frame.in_loop {
                self.stack_frames[self.stack_ptr as usize] = IterState {
                    i: current_frame.i,
                    in_loop: true,
                    depth: current_frame.depth,
                };
                self.stack_ptr += 1;
                continue;
            }

            // If we have no idea what i is because we haven't set the loop, we know to start the loop now
            if current_frame.i >= (current_frame.depth - 1) as isize {
                self.stack_frames[self.stack_ptr as usize] = IterState::new(current_frame.depth);
                if self.stack_ptr >= 1 {
                    self.stack_ptr -= 1;
                }
                continue;
            }

            if current_frame.i == -1 {
                current_frame.i = 0;
            }

            let swap_index = if current_frame.depth % 2 == 0 {
                current_frame.i
            } else {
                0
            };
            self.current
                .swap(swap_index as usize, current_frame.depth - 1);
            self.stack_frames[self.stack_ptr as usize] = IterState {
                i: current_frame.i + 1,
                depth: current_frame.depth,
                in_loop: true,
            };
            self.stack_ptr += 1;
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
