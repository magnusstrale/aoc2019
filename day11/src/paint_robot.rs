use std::collections::HashMap;
use std::cmp;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self { Self { x, y} }
}

//           ^ +Y
//           !
//           !
// -X <------+-------> +X 
//           !
//           !
//           v -Y
pub struct PaintRobot {
    position: Point,
    dx: isize,
    dy: isize, 
    painted_info: HashMap<Point, isize>
}

impl PaintRobot {
    pub fn new() -> Self { 
        Self { position: Point::new(0, 0), dx: 0, dy: 1, painted_info: HashMap::new() }
    }

    fn turn_left(&mut self) {
        if self.dy == 1 { self.dx = -1; self.dy = 0; return; }
        if self.dx == -1 { self.dx = 0; self.dy = -1; return; }
        if self.dy == -1 { self.dx = 1; self.dy = 0; return; }
        self.dx = 0; self.dy = 1;
    }

    fn turn_right(&mut self) {
        if self.dy == 1 { self.dx = 1; self.dy = 0; return; }
        if self.dx == 1 { self.dx = 0; self.dy = -1; return; }
        if self.dy == -1 { self.dx = -1; self.dy = 0; return; }
        self.dx = 0; self.dy = 1;
    }

    fn turn_robot(&mut self, turn: isize) {
        match turn {
            0 => self.turn_left(),
            1 => self.turn_right(),
            _ => panic!("Invalid direction {}", turn)
        }
    }

    fn move_robot(&mut self) {
        self.position.x += self.dx;
        self.position.y += self.dy;
    }

    pub fn turn_and_move(&mut self, direction: isize) {
        self.turn_robot(direction);
        self.move_robot();
    }

    pub fn paint_here(&mut self, color: isize) {
        let spot = self.painted_info.entry(self.position).or_insert(0);
        *spot = color;
    }

    pub fn get_color_here(&self) -> isize {
        *self.painted_info.get(&self.position).unwrap_or(&0)
    }

    pub fn colored_panels(&self) -> usize {
        self.painted_info.len()
    }

    fn get_min_max_coord(&self) -> (Point, Point) {
        let mut min = Point::new(0, 0);
        let mut max = Point::new(0, 0);
        for (p, _) in &self.painted_info {
            min.x = cmp::min(min.x, p.x);
            min.y = cmp::min(min.y, p.y);
            max.x = cmp::max(max.x, p.x);
            max.y = cmp::max(max.y, p.y);
        }

        (min, max)
    }

    pub fn print_paint(&self) {
        let (min, max) = self.get_min_max_coord();
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let color = *self.painted_info.get(&Point::new(x, max.y - y + min.y)).unwrap_or(&0);
                if color == 0 { print!(" ") } else { print!("X") }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_trivial_program_should_paint_1_spot() {
        let mut sut = PaintRobot::new();

        assert_eq!(0, sut.get_color_here());
        assert_eq!(0, sut.colored_panels());
        sut.paint_here(1);
        sut.turn_and_move(0);

        assert_eq!(Point::new(-1, 0), sut.position);
        let spot = sut.painted_info.get(&Point::new(0,0)).unwrap();
        assert_eq!(1, *spot);

        assert_eq!(1, sut.colored_panels());
    }

    #[test]
    fn execute_four_left_turn_and_validate_positions() {
        let mut sut = PaintRobot::new();

        assert_eq!(Point::new(0, 0), sut.position);
        sut.turn_and_move(0);
        assert_eq!(Point::new(-1, 0), sut.position);
        sut.turn_and_move(0);
        assert_eq!(Point::new(-1, -1), sut.position);
        sut.turn_and_move(0);
        assert_eq!(Point::new(0, -1), sut.position);
        sut.turn_and_move(0);
        assert_eq!(Point::new(0, 0), sut.position);
    }
    #[test]
    fn execute_four_right_turn_and_validate_positions() {
        let mut sut = PaintRobot::new();

        assert_eq!(Point::new(0, 0), sut.position);
        sut.turn_and_move(1);
        assert_eq!(Point::new(1, 0), sut.position);
        sut.turn_and_move(1);
        assert_eq!(Point::new(1, -1), sut.position);
        sut.turn_and_move(1);
        assert_eq!(Point::new(0, -1), sut.position);
        sut.turn_and_move(1);
        assert_eq!(Point::new(0, 0), sut.position);
    }
}