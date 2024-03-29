use std::cmp;
use std::usize;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self { Self { x, y} }
    fn x_min(&self, other: Point) -> isize { cmp::min(self.x, other.x) }
    fn x_max(&self, other: Point) -> isize { cmp::max(self.x, other.x) }
    fn y_min(&self, other: Point) -> isize { cmp::min(self.y, other.y) }
    fn y_max(&self, other: Point) -> isize { cmp::max(self.y, other.y) }

    fn is_between(&self, p1: Point, p2: Point) -> bool {
        let (dx, dy) = p1.delta(p2);
        if dx == 0 || dy == 0 { return true; }    // on horizontal/vertical line 

        let (dx_s, dy_s) = p1.delta(*self);
        if dx_s == 0 || dy_s == 0 { return false; } // self is on horizontal/vertical line from p1, but we know that there is no horizontal/vertical line p1 to p2
        
        // Crosswise extension by denominator - 
        dx*dy_s == dy*dx_s
    }

    fn delta(&self, other: Point) -> (isize, isize) {
        let dx = (self.x as isize) - (other.x as isize);
        let dy = (self.y as isize) - (other.y as isize);
        (dx, dy)
    }

    fn value_clockwise_increasing(&self, base: Point) -> isize {
        // Need to scale and convert into isize since f64 does not implement Ord and thus cannot have 
        // full ordering (looking at you, ugly-ass IEEE fp standard). For the purpose here this workaround
        // is enough, since all we want to know is the relative angles.
        // minimum is at Y > 0, X = 0
       ((-(self.x as f64 - base.x as f64)).atan2(self.y as f64 - base.y as f64) * 10_000_f64) as isize
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    asteroid: Point,
    dx: isize,
    dy: isize,
//    slope: f64
}

impl Vector {
    pub fn new(base: Point, asteroid: Point) -> Self { 
        let (dx, dy) = asteroid.delta(base);
        Vector { asteroid, dx, dy }
    }

}

#[derive(Debug)]
pub struct Map {
    width: usize,
    height: usize,
    map: Vec<Vec<bool>>
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
        Map { map, width: map_data[0].len(), height: map_data.len() }
    }

    fn asteroids_in_area(&self, corner1: Point, corner2: Point) -> Vec<Point> {
        let mut asteroids = Vec::new();

        for y in corner1.y_min(corner2)..=corner1.y_max(corner2) {
            for x in corner1.x_min(corner2)..=corner1.x_max(corner2) {
                if self.map[y as usize][x as usize] { asteroids.push(Point::new(x, y)); }
            }
        }
        asteroids
    }

    fn all_asteroids(&self) -> Vec<Point> {
        self.asteroids_in_area(Point::new(0, 0), Point::new((self.width - 1) as isize, (self.height - 1) as isize))
    }

    fn is_visible_from(&self, base: Point, asteroid: Point) -> bool {
        let maybe_blocking = self.asteroids_in_area(base, asteroid).into_iter().filter(|a| *a != base && *a != asteroid);
        maybe_blocking.into_iter().all(|blocking_asteroid| !blocking_asteroid.is_between(base, asteroid))
    }

    pub fn find_best_location(&self) -> (Point, usize) {
        let mut max_count = 0;
        let mut best_location = Point::new(0, 0);

        let all_asteroids = self.all_asteroids();
        for base in &all_asteroids {
            let mut count = 0;
            for asteroid in &all_asteroids {
                if asteroid == base { continue; }
                if self.is_visible_from(*base, *asteroid) { count += 1; }
            }
            if count > max_count { max_count = count; best_location = *base; }
        }

        (best_location, max_count)
    }

    fn all_vectors_from(&self, base: Point) -> Vec<Vector> {
        self.all_asteroids().into_iter().filter(|a| *a != base).map(|asteroid| Vector::new(base, asteroid)).collect()
    }

    pub fn vaporize_asteroids(&mut self, base: Point) -> Vec<Point> {
        let mut vaporized: Vec<Point> = Vec::new();
        let asteroid_count = self.all_asteroids().len();
        while vaporized.len() < asteroid_count - 1 {
            for gone in &vaporized {
                self.map[gone.y as usize][gone.x as usize] = false;
            }
            let mut all_asteroid_vectors: Vec<Vector> = self.all_vectors_from(base);
            all_asteroid_vectors.sort_by_key(|v| v.asteroid.value_clockwise_increasing(base));
            for vector in all_asteroid_vectors {
                if self.is_visible_from(base, vector.asteroid) { vaporized.push(vector.asteroid); }
            }
        }

        vaporized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atan2_and_stuff() {
        // We want to use atan2 to give 0 for y > 0, x = 0, pi/2 for y = 0, x > 0, pi/2 for 
    }
    #[test]
    fn part2_sample2_should_vaporize_36() {
        let mut sut = Map::new(&vec![
        ".#....#####...#..",   // 7
        "##...##.#####..##",   // 11
        "##...#...#.#####.",   // 9
        "..#.....#...###..",   // 5
        "..#.#.....#....##"]); // 5
        
        let vaporized = sut.vaporize_asteroids(Point::new(8,3));

        assert_eq!(36, vaporized.len());
        assert_eq!(Point::new(8, 1), vaporized[0]);
        assert_eq!(Point::new(13, 2), vaporized[10]);
        assert_eq!(Point::new(15, 2), vaporized[12]);
        assert_eq!(Point::new(16, 4), vaporized[14]);
        assert_eq!(Point::new(10, 4), vaporized[16]);
        assert_eq!(Point::new(4, 4), vaporized[17]);
        assert_eq!(Point::new(2, 3), vaporized[19]);
        assert_eq!(Point::new(7, 0), vaporized[29]);
        assert_eq!(Point::new(14, 3), vaporized[35]);
    }

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