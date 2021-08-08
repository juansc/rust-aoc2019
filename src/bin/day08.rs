use aoc2019::util;

fn main() {
    let lines = util::lines_from_file("./input/day08.txt");
    println!("Solution for Part 1: {}", part1(lines.as_slice(), 25, 6));
    println!("Solution for Part 2:\n {}", part2(lines.as_slice(), 25, 6));
}

fn part1(line: &[String], height: usize, width: usize) -> usize {
    let image = Image::new(line, height, width);
    // Go through all the layers. The accumulator here is (index, value), and we keep the minimum
    // value and the layer it happened. It's may not be faster, but it was an interesting way
    // to write it as an iterator
    let layer_with_least_zeroes = (0..image.layers)
        .fold((0, usize::MAX), |acc, i| {
            let val = image.digits_in_layer(i, 0);
            if val < acc.1 {
                (i, val)
            } else {
                acc
            }
        })
        .0;
    image.digits_in_layer(layer_with_least_zeroes, 1)
        * image.digits_in_layer(layer_with_least_zeroes, 2)
}

fn part2(line: &[String], height: usize, width: usize) -> String {
    Image::new(line, height, width).image_string()
}

struct Image {
    pixels: Vec<u8>,
    height: usize,
    width: usize,
    layer_size: usize,
    layers: usize,
}

impl Image {
    fn new(input: &[String], height: usize, width: usize) -> Self {
        let pixels: Vec<u8> = input
            .get(0)
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10))
            .inspect(|n| {
                if n.is_none() {
                    panic!("could not parse digit")
                }
            })
            .map(|n| n.unwrap() as u8)
            .collect();

        let layer_size = height * width;
        let num_pixels = pixels.len();
        if num_pixels % (layer_size) != 0 {
            panic!("the image has {} digits, but has height={} and width={} and {}x{}={} which does not evenly divide into layers", input.len(), height, width, height, width, height * width);
        }
        Self {
            pixels,
            height,
            width,
            layer_size,
            layers: num_pixels / layer_size,
        }
    }

    fn digits_in_layer(&self, layer: usize, digit: u8) -> usize {
        self.pixels[self.layer_size * layer..self.layer_size * (layer + 1)]
            .iter()
            .filter(|n| **n == digit)
            .count()
    }

    fn pixel(&self, layer: usize, x: usize, y: usize) -> Option<u8> {
        if x > self.width || y > self.height || layer > self.layers {
            return None;
        }
        Some(self.pixels[layer * self.layer_size + x + y * self.width])
    }

    fn image_string(&self) -> String {
        let mut out: String = String::from("");
        for y in 0..self.height {
            for x in 0..self.width {
                match self.color_for_pixel(x, y) {
                    Color::Black => {
                        out.push_str(&String::from(" "));
                    }
                    Color::White => {
                        out.push_str(&String::from("*"));
                    }
                    _ => (),
                }
            }
            out.push_str(&String::from("\n"));
        }
        out
    }

    fn color_for_pixel(&self, x: usize, y: usize) -> Color {
        for layer in 0..self.layers {
            match self.pixel(layer, x, y).unwrap() {
                0 => return Color::Black,
                1 => return Color::White,
                _ => continue,
            }
        }
        Color::Transparent
    }
}

enum Color {
    Black,
    White,
    Transparent,
}

// Tests

#[cfg(test)]
mod tests {
    use crate::util;
    use crate::{part1, part2};

    #[test]
    fn test_part1_simple() {
        let lines = &[String::from("000111222033111322")];
        assert_eq!(part1(lines, 3, 3), 6);
    }

    #[test]
    fn test_part1() {
        let lines = util::lines_from_file("./input/day08.txt");
        assert_eq!(part1(&lines, 6, 25), 2193);
    }

    #[test]
    fn test_part2_simple() {
        let lines = &[String::from(
            "222222222222222222222222111111000000000000000000000000",
        )];
        let expected = concat!("      \n", "******\n", "      \n");
        assert_eq!(part2(lines, 3, 6), expected);
    }

    #[test]
    fn test_part2() {
        let lines = util::lines_from_file("./input/day08.txt");
        // Should print out string that says YEHEF
        let expected = concat!(
            "*   ***** *  * **** **** \n",
            "*   **    *  * *    *    \n",
            " * * ***  **** ***  ***  \n",
            "  *  *    *  * *    *    \n",
            "  *  *    *  * *    *    \n",
            "  *  **** *  * **** *    \n"
        );
        assert_eq!(part2(&lines, 6, 25), expected);
    }
}
