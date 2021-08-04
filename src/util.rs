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

struct Permutations {
    counter: u32,
    current: Vec<u32>,
    limit: u32,
    stack_frames: Vec<IterState>,
    stack_ptr: u32,
}

impl Permutations {
    fn new(n: usize) -> Self {
        let mut arr = vec![0u32; n];
        let mut stack_frames = Vec::with_capacity(n);
        let mut limit = 1u32;
        for i in 0..n {
            arr[i] = i as u32;
            limit *= (i as u32) + 1;
            stack_frames.push(IterState {
                i: -1,
                depth: n - i,
                in_loop: false,
            });
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

impl Iterator for Permutations {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        // Once we have exceeded the number of possible permutations
        // return None
        if self.counter >= self.limit {
            println!("counter={} limit={}", self.counter, self.limit);
            return None;
        }
        println!("ptr={}", self.stack_ptr);

        loop {
            println!("\n\nentering fn");
            let f = self.stack_frames.get(self.stack_ptr as usize).unwrap();
            let mut current_frame = IterState {
                i: f.i,
                depth: f.depth,
                in_loop: f.in_loop,
            };
            println!(
                "depth={} ptr={} limit={} counter={} i={}",
                current_frame.depth, self.stack_ptr, self.limit, self.counter, current_frame.i,
            );

            if current_frame.depth == 1 {
                println!("Returning because at depth 1");
                self.stack_frames[self.stack_ptr as usize] = IterState {
                    i: -1,
                    in_loop: false,
                    depth: current_frame.depth,
                };
                if self.stack_ptr >= 1 {
                    self.stack_ptr -= 1;
                }
                self.counter += 1;
                println!("{:?}", self.current);
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
            if current_frame.i == -1 {
                println!("haven't done the loop yet, initializing i to 0");
                current_frame.i = 0;
            } else if current_frame.i >= (current_frame.depth - 1) as isize {
                println!("Finished for loop");
                self.stack_frames[self.stack_ptr as usize] = IterState {
                    i: -1,
                    in_loop: false,
                    depth: current_frame.depth,
                };
                self.stack_ptr -= 1;
                continue;
            }

            println!("doing the swap");
            if current_frame.depth % 2 == 0 {
                self.current
                    .swap(current_frame.i as usize, current_frame.depth - 1);
            } else {
                self.current.swap(0, current_frame.depth - 1);
            }
            println!(
                "Updating current frame state i={} depth={} i_new={}",
                current_frame.i,
                current_frame.depth,
                current_frame.i + 1,
            );
            self.stack_frames[self.stack_ptr as usize] = IterState {
                i: current_frame.i + 1,
                depth: current_frame.depth,
                in_loop: true,
            };
            self.stack_frames[self.stack_ptr as usize + 1] = IterState {
                i: -1,
                depth: current_frame.depth - 1,
                in_loop: false,
            };
            self.stack_ptr += 1;
        }
    }
}

pub fn nonrperms(input: &mut Vec<i32>) -> Vec<Vec<i32>> {
    let mut stack_state: Vec<usize> = Vec::new();
    let mut out = Vec::new();
    out.push(input.clone());

    let mut stack_ptr = 1;
    let n = input.len();
    while stack_ptr < n {
        if stack_state[stack_ptr] < stack_ptr {
            if stack_ptr % 2 == 0 {
                input.swap(0, stack_ptr)
            } else {
                input.swap(stack_state[stack_ptr], stack_ptr)
            }
            out.push(input.clone());
        } else {
            stack_state[stack_ptr] = 0;
            stack_ptr += 1;
        }
    }
    out
}

pub fn perms(k: usize, input: &mut Vec<i32>) -> Vec<Vec<i32>> {
    let mut out: Vec<Vec<i32>> = Vec::new();
    if k == 1 {
        return vec![input.clone()];
    }
    let out_perms = perms(k - 1, input);
    for e in out_perms {
        out.push(e.clone());
    }
    for i in 0..k - 1 {
        if k % 2 == 0 {
            input.swap(i, k - 1);
        } else {
            input.swap(0, k - 1);
        }
        let out_perms = perms(k - 1, input);
        for e in out_perms {
            out.push(e.clone());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use crate::util::{perms, Permutations};

    #[test]
    fn test_perm_iterator() {
        assert_eq!(Permutations::new(1).into_iter().count(), 1);
        assert_eq!(Permutations::new(2).into_iter().count(), 2);
        assert_eq!(Permutations::new(3).into_iter().count(), 6);
        assert_eq!(Permutations::new(4).into_iter().count(), 24);
    }

    #[test]
    fn test_single_elem_permuration() {
        let mut elements = vec![1i32];
        assert_eq!(vec![vec![1i32]], perms(elements.len(), &mut elements));
    }

    #[test]
    fn test_double_elem_permuration() {
        let mut elements = vec![0i32, 1i32];

        assert_eq!(
            vec![vec![0i32, 1i32], vec![1i32, 0i32]],
            perms(elements.len(), &mut elements)
        );
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
