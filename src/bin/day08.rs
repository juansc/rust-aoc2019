use aoc2019::util;

fn main() {
    let lines = util::lines_from_file("./input/day08.txt");
    println!("Solution for Part 1: {}", part1(lines.as_slice(), 25, 6));
}

fn part1(line: &[String], height: usize, width: usize) -> usize {
    let image = Image::new(line, height, width);
    let mut layer_with_least_zeroes = 0;
    let mut min_zeroes = usize::max_value();
    for i in 0..image.layers {
        let zeroes = image.digits_in_layer(i, 0);
        if zeroes < min_zeroes {
            min_zeroes = zeroes;
            layer_with_least_zeroes = i;
        }
    }
    image.digits_in_layer(layer_with_least_zeroes, 1)
        * image.digits_in_layer(layer_with_least_zeroes, 2)
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
        let mut n = 0;
        for i in self.layer_size * layer..self.layer_size * (layer + 1) {
            if *self.pixels.get(i).unwrap() == digit {
                n += 1;
            }
        }
        n
    }

    fn row(&self, layer: usize, n: usize) -> Option<&[u8]> {
        if layer - 1 > self.layers {
            return None;
        }
        let start = self.layer_size * layer + self.width * n;
        let end = start + self.width;
        Some(&self.pixels[start..end])
    }

    fn pixel(&self, layer: usize, x: usize, y: usize) -> Option<u8> {
        if x - 1 > self.width || y - 1 > self.height || layer - 1 > self.layers {
            return None;
        }
        Some(self.pixels[layer * self.layer_size + x + y * self.height])
    }
}

// Tests

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::util;

    #[test]
    fn test_fuel_for_mass() {
        assert_eq!(1, 1);
    }

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
}
