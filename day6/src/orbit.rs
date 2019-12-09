use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Orbit {
    name: String,
    center: String
}

#[derive(Debug, Clone)]
pub struct OrbitMap {
    orbits: HashMap<String, Orbit>
}

impl OrbitMap {
    pub  fn new(orbits_input: &Vec<&str>) -> Self {
        let mut orbits = HashMap::new();
        for line in orbits_input {
            let items: Vec<&str> = line.split(')').collect();
            let center = items[0];
            let name = items[1];
            let orb = Orbit { name: name.to_string(), center: center.to_string() };
            orbits.insert(orb.name.clone(), orb);
        }
        OrbitMap {orbits: orbits}
    }

    fn orbits(&self, orbit: &Orbit) -> usize {
        if orbit.center == "COM".to_string() { return 1; }
        let new_orbit = self.orbits.get(&orbit.center).unwrap();
        1 + self.orbits(new_orbit)
    }
    
    pub fn total_orbits(&self) -> usize {
        self.orbits.values().map(|o| self.orbits(o)).sum()
    }

    fn orbit_path<'a>(&'a self, orbit: &'a Orbit) -> HashSet<&'a Orbit> {
        let mut path = HashSet::new();
        path.insert(orbit);
        if orbit.center != "COM".to_string() { 
            let new_orbit = self.orbits.get(&orbit.center).unwrap();
            path.extend(self.orbit_path(new_orbit));
        }
        path
    }

    fn orbit_path_from_name(&self, name: &str) -> HashSet<&Orbit> {
        let orbit = self.orbits.get(&name.to_string()).unwrap();
        self.orbit_path(orbit)
    }

    pub fn distance(&self) -> usize {
        let you_set = self.orbit_path_from_name("YOU");
        let san_set = self.orbit_path_from_name("SAN");
        let diff: HashSet<_> = you_set.symmetric_difference(&san_set).collect();
        diff.len() - 2 // Don't cound end nodes, just the path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1_part1_d_orbits_3() {
        let input = vec!["COM)B", "B)C", "C)D", "D)E","E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L" ];
        let map = OrbitMap::new(&input);

        let start = map.orbits.get(&"D".to_string()).unwrap();
        assert_eq!(3, map.orbits(start));

        let start = map.orbits.get(&"L".to_string()).unwrap();
        assert_eq!(7, map.orbits(start));
    }

    #[test]
    fn sample1_part1_should_give_42() {
        let input = vec!["COM)B", "B)C", "C)D", "D)E","E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L" ];
        let map = OrbitMap::new(&input);
        println!("{:?}", map);
        assert_eq!(42, map.total_orbits());
    }

    #[test]
    fn sample1_part1_reordered_should_give_42() {
        let input = vec!["B)C", "C)D", "D)E","E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "COM)B" ];
        let map = OrbitMap::new(&input);
        assert_eq!(42, map.total_orbits());
    }

    #[test]
    fn sample1_part2_distance_me_to_santa_is_4() {
        let input = vec!["COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU", "I)SAN"];
        let map = OrbitMap::new(&input);

        assert_eq!(4, map.distance());
    }
}