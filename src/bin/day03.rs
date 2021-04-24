use std::cmp::{max, min};

use aoc2019::util;

// Here we make Point derive both copy and clone. This way when we need to pass a point around
// we actually make a deep copy, which is fine. We also implment Eq to allow us to compare points
// easily, and to then compare segments easily.
#[derive(Copy, Clone, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        return Point { x, y };
    }

    fn l1_distance(&self, other: Point) -> u32 {
        ((self.y - other.y).abs() + (self.x - other.x).abs()) as u32
    }

    fn add(&self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

/// Segment represent a horizontal or straight line in Taxicab geometry. We use this assumption
/// to simplify some geometry such as detecting whether two segments are parallel.
#[derive(PartialEq, Debug)]
struct Segment {
    a: Point,
    b: Point,
}

impl Segment {
    fn new(a: Point, b: Point) -> Segment {
        Segment { a, b }
    }

    fn contains(&self, p: &Point) -> bool {
        let x_min = min(self.a.x, self.b.x);
        let x_max = max(self.a.x, self.b.x);
        let y_min = min(self.a.y, self.b.y);
        let y_max = max(self.a.y, self.b.y);
        return x_min <= p.x && p.x <= x_max && y_min <= p.y && p.y <= y_max;
    }

    fn intersection(&self, other: &Segment) -> Option<Point> {
        // If two lines are parallel they either do not intersect or intersect at many points
        if self.is_parallel_to(other) {
            return None;
        }
        // We know one segment is horizontal and one is vertical. The intersection point, if it
        // exists, has the y-value of the horizontal segment and the x-value of the vertical
        // segment. The point must be contained in both segments for it so exist.
        let p;
        if self.is_horizontal() {
            p = Point { x: other.a.x, y: self.a.y }
        } else {
            p = Point { x: self.a.x, y: other.a.y }
        }
        if !self.contains(&p) || !other.contains(&p) {
            return None;
        }
        // If the intersecion point is an endpoint on both segments then the wires
        // just intersected at an elbow, which doesn't count
        if (p == self.a || p == self.b) && (p == other.a || p == other.b) {
            return None;
        }
        Some(p)
    }

    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    fn is_parallel_to(&self, other: &Segment) -> bool {
        self.is_vertical() == other.is_vertical() && self.is_horizontal() == other.is_horizontal()
    }
}

fn wire_segments(input: &str) -> Vec<Segment> {
    let mut p = Point::new(0, 0);
    let mut new_point;
    let mut segments = vec![];
    for line in input.split(",") {
        // Grab just the first character
        let direction = &line[0..1];
        // Grab the rest of the string and parse to i32
        let size = (&line[1..]).parse::<i32>().unwrap();
        match direction {
            "R" => {
                new_point = p.add(Point::new(size, 0))
            }
            "D" => {
                new_point = p.add(Point::new(0, -size))
            }
            "L" => {
                new_point = p.add(Point::new(-size, 0))
            }
            "U" => {
                new_point = p.add(Point::new(0, size))
            }
            _ => panic!("invalid direction {}", direction),
        }
        segments.push(Segment::new(p, new_point));
        p = new_point;
    }
    segments
}

fn get_intersections(segments1: Vec<&Segment>, segments2: Vec<&Segment>) -> Vec<Point> {
    let mut intersections: Vec<Point> = Vec::new();
    for seg1 in segments1.iter() {
        for seg2 in segments2.iter() {
            match seg1.intersection(seg2) {
                Some(p) => {
                    intersections.push(p)
                }
                None => {}
            }
        }
    }
    intersections
}

fn part1(lines: Vec<String>) -> u32 {
    let wire1 = wire_segments(&lines[0]);
    let wire2 = wire_segments(&lines[1]);
    let origin = Point::new(0, 0);

    let wire1_h: Vec<&Segment> = wire1.iter().filter(|l| l.is_horizontal()).collect();
    let wire2_h: Vec<&Segment> = wire2.iter().filter(|l| l.is_horizontal()).collect();
    let wire1_v: Vec<&Segment> = wire1.iter().filter(|l| l.is_vertical()).collect();
    let wire2_v: Vec<&Segment> = wire2.iter().filter(|l| l.is_vertical()).collect();

    // Collect all the intersections...
    get_intersections(wire1_h, wire2_v).
        into_iter().
        chain(get_intersections(wire1_v, wire2_h)).
        // An intersection is only valid if it's not the origin
        filter(|p| *p != origin).
        // Get the distance that is the closest to the origin
        map(|p| p.l1_distance(origin)).
        min().
        // Panic if no intersections were found. Given the scope of this project, we should
        // just panic instead of handling this gracefully.
        expect("no intersections found") as u32
}

fn main() {
    let lines = util::lines_from_file("./input/day03.txt");
    println!("Part 1 Solution: {}", part1(lines))
}

#[cfg(test)]
mod tests {
    use aoc2019::util;

    use crate::{part1, Point, Segment, wire_segments};

    #[test]
    fn test_point_l1_distance() {
        let p1 = Point::new(0, 0);
        assert_eq!(p1.l1_distance(Point::new(10, 0)), 10);
        assert_eq!(p1.l1_distance(Point::new(0, 10)), 10);
        assert_eq!(p1.l1_distance(Point::new(5, 5)), 10);
    }

    #[test]
    fn test_segment_horizontal_and_vertical() {
        struct SegmentTest {
            name: String,
            segment: Segment,
            is_horizontal: bool,
        }
        let tests = vec![
            SegmentTest {
                name: "Vertical Line".to_string(),
                segment: Segment::new(
                    Point::new(0, 0),
                    Point::new(0, 5),
                ),
                is_horizontal: false,
            },
            SegmentTest {
                name: "Horizontal Line".to_string(),
                segment: Segment::new(
                    Point::new(1, 0),
                    Point::new(4, 0),
                ),
                is_horizontal: true,
            }
        ];
        for test in tests {
            assert_eq!(test.segment.is_horizontal(), test.is_horizontal, "{}", test.name);
            assert_eq!(test.segment.is_vertical(), !test.is_horizontal, "{}", test.name);
        }
    }

    #[test]
    fn test_segment_contains_point() {
        let segment = Segment::new(
            Point::new(0, 0),
            Point::new(0, 5),
        );
        assert_eq!(segment.contains(&Point::new(0, 4)), true);
        assert_eq!(segment.contains(&Point::new(1, 4)), false);
    }

    #[test]
    fn test_wire_segments() {
        let input = "R30,U50,L40,D50,R10";
        let out = wire_segments(input);
        let expected = vec![
            Segment::new(Point::new(0, 0), Point::new(30, 0)),
            Segment::new(Point::new(30, 0), Point::new(30, 50)),
            Segment::new(Point::new(30, 50), Point::new(-10, 50)),
            Segment::new(Point::new(-10, 50), Point::new(-10, 0)),
            Segment::new(Point::new(-10, 0), Point::new(0, 0)),
        ];
        assert_eq!(out.len(), expected.len());
        for (i, e) in out.iter().enumerate() {
            assert_eq!(e, &expected[i])
        }
    }

    #[test]
    fn test_part1() {
        let lines = util::lines_from_file("./input/day03.txt");
        assert_eq!(part1(lines), 375)
    }

    #[test]
    fn test_part1_basic() {
        let lines = vec![String::from("R8,U5,L5,D3"), String::from("U7,R6,D4,L4")];
        assert_eq!(part1(lines), 6);

        let lines = vec![
            String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            String::from("U62,R66,U55,R34,D71,R55,D58,R83"),
        ];
        assert_eq!(part1(lines), 159);

        let lines = vec![
            String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        ];
        assert_eq!(part1(lines), 135);
    }
}