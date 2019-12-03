use std::cmp;

// Coordinates are 
//                +Y
//                ^
//                |
//  -X  <---------+-------->  +X
//                |
//                v
//               -Y 

#[derive(Debug, Clone, Copy)]
struct Segment {
    x_start: i64,
    y_start: i64,
    x_stop: i64,
    y_stop: i64
}

impl Segment {
    pub fn new(start: &Segment, instruction: &str) -> Self {
        let ch: Vec<char> = instruction.chars().collect();
        let (delta_x, delta_y) = match ch[0] {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!("Invalid instruction {}", ch[0])
        };
        let distance: i64 = (&instruction[1..]).parse().unwrap();
        let s = Segment { 
            x_start: start.x_stop,
            y_start: start.y_stop,
            x_stop: start.x_stop + distance * delta_x,
            y_stop: start.y_stop + distance * delta_y};
        if (s.x_start != s.x_stop) && (s.y_start != s.y_stop) { panic!("Diagonal line detected {:?}", s); }
        s
    }
    pub fn first(instruction: &str) -> Self {
        Segment::new( &Segment { x_start: 0, y_start: 0, x_stop: 0, y_stop: 0}, instruction)
    }

    fn is_horizontal(&self) -> bool {
        self.y_start == self.y_stop
    }

    fn is_vertical(&self) -> bool {
        !self.is_horizontal()
    }

    fn is_between(val: i64, bound1: i64, bound2: i64) -> bool {
        val < cmp::max(bound1, bound2) && val > cmp::min(bound1, bound2)
    }

    pub fn intersect(&self, segment: &Segment) -> Option<(i64, i64)> {
        // Make sure that segments run in different planes (if both are horizontal or both vertical, they cannot intersect), otherwise exit
        if self.is_horizontal() && segment.is_horizontal() { return None; }
        if self.is_vertical() && segment.is_vertical() { return None; }
        let (vert, horiz) = if self.is_vertical() { (self, segment) } else { (segment, self) };

        // Create potential intersection by creating point from horizontal lines y-value and vertical lines x-value
        let (x, y) = (vert.x_start, horiz.y_start);

        // Check if the vertical line has its y-coordinates between the potential intersection
        // Check if the horizontal line has its x-coordinates between the potential intersection
        if Segment::is_between(y, vert.y_start, vert.y_stop) && Segment::is_between(x, horiz.x_start, horiz.x_stop) {
            println!("Intersection at {}, {}", x, y);
            return Some((x, y));
        }

        None
    }
}

pub struct Wire {
    segments: Vec<Segment>
}

impl Wire {
    pub fn new(line: &str) -> Self {
        let mut segs = Vec::new();
        let mut last_segment = Segment { x_start: 0, y_start: 0, x_stop: 0, y_stop: 0 };
        for instruction in line.split(',') {
            let new_seg = Segment::new(&last_segment, instruction);
            segs.push(new_seg);
            last_segment = new_seg;
        }
        Wire { segments: segs }
    }

    fn manhattan_distance(x: i64, y: i64) -> i64 {
        x.abs() + y.abs()
    }

    fn intersection_points_distance(&self, segment: &Segment) -> Vec<i64> {
        let mut points = Vec::new();
        for seg in &self.segments {
            match seg.intersect(segment) {
                Some((x, y)) => points.push(Wire::manhattan_distance(x, y)),
                None => ()
            }
        }
        points
    }

    pub fn min_intersection_distance(&self, other: &Wire) -> i64 {
        let mut distance: Vec<i64> = Vec::new();
        for s in &other.segments {
            let mut dist_for_seg = self.intersection_points_distance(&s);
            distance.append(&mut dist_for_seg);
        }
        distance.into_iter().fold(9999, |min, x| cmp::min(min, x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_with_up_5_gives_0_5() {
        let s = Segment::first("U5");
        assert_eq!((s.x_start, s.y_start, s.x_stop, s.y_stop), (0, 0, 0, 5));
    }

    #[test]
    fn first_with_down_3_gives_0_minus3() {
        let s = Segment::first("D3");
        assert_eq!((s.x_start, s.y_start, s.x_stop, s.y_stop), (0, 0, 0, -3));
    }

    #[test]
    fn first_with_left_368_gives_minus368_0() {
        let s = Segment::first("L368");
        assert_eq!((s.x_start, s.y_start, s.x_stop, s.y_stop), (0, 0, -368, 0));
    }

    #[test]
    fn first_with_right_256_gives_256_0() {
        let s = Segment::first("R256");
        assert_eq!((s.x_start, s.y_start, s.x_stop, s.y_stop), (0, 0, 256, 0));
    }

    #[test]
    fn new_from_1_2_3_4_with_up_5_gives_3_4_3_9() {
        let s = Segment::new( &Segment { x_start: 1, y_start: 4, x_stop: 3, y_stop: 4 }, "U5");
        assert_eq!((s.x_start, s.y_start, s.x_stop, s.y_stop), (3, 4, 3, 9));

    }

    #[test]
    fn wire_with_U1_L2_D3_R4_should_have_four_segments_and_end_up_at_3_minus2() {
        let w = Wire::new("U1,L2,D3,R4");

        assert_eq!(w.segments.len(), 4);
        assert_eq!(w.segments[3].x_stop, 2);
        assert_eq!(w.segments[3].y_stop, -2);
    }

    #[test]
    fn no_segment_intersection_gives_none() {
        let s1 = Segment { x_start: 0, y_start: 0, x_stop: 100, y_stop: 0 }; // Horizontal line stretching to right
        let s2 = Segment { x_start: 0, y_start: 0, x_stop: -1, y_stop: 0 }; // Horizntal line to left, level with s1

        assert_eq!(s1.intersect(&s2), None);
        assert_eq!(s2.intersect(&s1), None); // Should be reflexive
    }

    #[test]
    fn segment_intersection_gives_some_50_0() {
        let s1 = Segment { x_start: 0, y_start: 0, x_stop: 100, y_stop: 0 }; // Horizontal line stretching to right
        let s2 = Segment { x_start: 50, y_start: -10, x_stop: 50, y_stop: 10 }; // Vertical line to right, cutting s1 in half

        assert_eq!(s1.intersect(&s2), Some((50, 0)));
        assert_eq!(s2.intersect(&s1), Some((50, 0))); // Should be reflexive
    }

    #[test]
    fn segment_on_top_gives_no_intersection_since_it_does_not_cross() {
        let s1 = Segment { x_start: 0, y_start: 0, x_stop: 100, y_stop: 0 }; // Horizontal line stretching to right
        let s2 = Segment { x_start: 50, y_start: 0, x_stop: 750, y_stop: 0 }; // Horizontal line to right over s1

        assert_eq!(s1.intersect(&s2), None);
        assert_eq!(s2.intersect(&s1), None); // Should be reflexive
    }
}