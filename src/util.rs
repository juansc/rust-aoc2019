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
    current: Vec<i32>,
    ind: u16,
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
    use crate::util::perms;

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
        let mut elements = vec![0i32, 1i32, 2i32];

        assert_eq!(
            vec![
                vec![0i32, 1i32, 2i32],
                vec![1i32, 0i32, 2i32],
                vec![2i32, 0i32, 1i32],
                vec![0i32, 2i32, 1i32],
                vec![1i32, 2i32, 0i32],
                vec![2i32, 1i32, 0i32],
            ],
            perms(elements.len(), &mut elements)
        );
    }
}
