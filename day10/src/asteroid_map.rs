use std::cmp;
use std::usize;


const EPSILON: f64 = 0.00001;

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: usize,
    y: usize
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self { Self { x, y} }
    fn x_min(&self, other: Point) -> usize { cmp::min(self.x, other.x) }
    fn x_max(&self, other: Point) -> usize { cmp::max(self.x, other.x) }
    fn y_min(&self, other: Point) -> usize { cmp::min(self.y, other.y) }
    fn y_max(&self, other: Point) -> usize { cmp::max(self.y, other.y) }

    fn is_between(&self, p1: Point, p2: Point) -> bool {
        if *self == p1 || *self == p2 { return false; } 
        let (dx, dy) = p1.delta(p2);
        if dx == 0 || dy == 0 { return true; }    // on horizontal/vertical line 

        let (dx_s, dy_s) = p1.delta(*self);
        if dx_s == 0 || dy_s == 0 { return false; } // self is on horizontal/vertical line from p1, but we know that there is no horizontal/vertical line p1 to p2
        
        approx_eq(dx as f64 / dy as f64, dx_s as f64 / dy_s as f64)
    }

    fn delta(&self, other: Point) -> (isize, isize) {
        let dx = (self.x as isize) - (other.x as isize);
        let dy = (self.y as isize) - (other.y as isize);
        (dx, dy)
    }
}

#[derive(Debug)]
pub struct Map {
    width: usize,
    height: usize,
    map: Vec<Vec<bool>>,
    debug: bool
}

impl Map {
    pub fn new(map_data: &Vec<&str>) -> Self {
        let mut map = Vec::new();
        for line in map_data {
            let mut row = Vec::new();
            for ch in line.chars() {
                row.push(ch == '#');
            }
            map.push(row);
        }
        Map { map: map, width: map_data[0].len(), height: map_data.len(), debug: false }
    }

    fn asteroids_in_area(&self, corner1: Point, corner2: Point) -> Vec<Point> {
        let mut asteroids = Vec::new();

        if self.debug { println!("Sub area from {:?} to {:?}", corner1, corner2); }
        for y in corner1.y_min(corner2)..=corner1.y_max(corner2) {
            for x in corner1.x_min(corner2)..=corner1.x_max(corner2) {
                if self.map[y][x] { asteroids.push(Point::new(x, y)); if self.debug { print!("#");} } else if self.debug { print!("."); }
            }
            if self.debug { println!(); }
        }
        asteroids
    }

    pub fn find_best_location(&self) -> (Point, usize) {
        let mut max_count = 0;
        let mut best_location = Point::new(0, 0);

        let all_asteroids = self.asteroids_in_area(Point::new(0, 0), Point::new(self.width - 1, self.height - 1));
        for base_location in &all_asteroids {
            let mut count = 0;
            for asteroid in &all_asteroids {
                if asteroid == base_location { continue; }
                let blocking = self.asteroids_in_area(*base_location, *asteroid);
                // Should filter out base_location & asteroid from blocking, but that's handled in is_between
                let is_visible = blocking.iter().all(|blocking_asteroid| !blocking_asteroid.is_between(*base_location, *asteroid));
                if is_visible { count += 1; }
            }
            if count > max_count { max_count = count; best_location = *base_location; }
        }

        (best_location, max_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_map_1_should_find_8_asteroids_from_3_4() {
        let sut = Map::new(&vec![
            ".#..#",
            ".....",
            "#####",
            "....#",
            "...##"]);

        let (location, count) = sut.find_best_location();
        assert_eq!(count, 8);

        assert_eq!(location.x, 3);
        assert_eq!(location.y, 4);
    }

    #[test]
    fn sample_map_2_should_find_33_asteroids_from_5_8() {
        let sut = Map::new(&vec![
            "......#.#.",
            "#..#.#....",
            "..#######.",
            ".#.#.###..",
            ".#..#.....",
            "..#....#.#",
            "#..#....#.",
            ".##.#..###",
            "##...#..#.",
            ".#....####"]);

        let (location, count) = sut.find_best_location();
        
        assert_eq!(count, 33);

        assert_eq!(location.x, 5);
        assert_eq!(location.y, 8);
    }

    #[test]
    fn sample_map_big_should_find_210_asteroids_from_11_13() {
        let sut = Map::new(&vec![
            ".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##"]);

        let (location, count) = sut.find_best_location();
        
        assert_eq!(count, 210);

        assert_eq!(location.x, 11);
        assert_eq!(location.y, 13);
    }

    #[test]
    fn sample_map_1_should_have_2_asteroids_in_area_1_0_1_2() {
        let sut = Map::new(&vec![
            ".#..#",
            ".....",
            "#####",
            "....#",
            "...##"]);

        let asteroids = sut.asteroids_in_area(Point::new(1, 0), Point::new(1, 2));

        assert_eq!(2, asteroids.len());
        assert_eq!(Point::new(1, 0), asteroids[0]);
        assert_eq!(Point::new(1, 2), asteroids[1]);
    }

    #[test]
    fn sample_map_1_should_have_10_asteroids_in_area_0_0_4_4() {
        let sut = Map::new(&vec![
            ".#..#",
            ".....",
            "#####",
            "....#",
            "...##"]);

        let asteroids = sut.asteroids_in_area(Point::new(0, 0), Point::new(4, 4));

        assert_eq!(10, asteroids.len());
    }

    #[test]
    fn should_find_that_3_1_is_between_0_0_and_6_2() {
        let base_location = Point::new(0, 0);
        let asteroid = Point::new(6, 2);
        let blocking = Point::new(3, 1);

        assert!(blocking.is_between(asteroid, base_location));
    }

    #[test]
    fn should_find_that_2_3_is_not_between_1_0_and_4_3() {
        let base_location = Point::new(1, 0);
        let asteroid = Point::new(4, 3);
        let blocking = Point::new(2, 2);

        assert!(!blocking.is_between(asteroid, base_location));
    }

    #[test]
    fn should_find_that_2_3_is_not_between_4_4_and_1_0() {
        let base_location = Point::new(4, 4);
        let asteroid = Point::new(1, 0);
        let blocking = Point::new(2, 2);

        assert!(!blocking.is_between(asteroid, base_location));
    }
}