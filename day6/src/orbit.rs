use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Orbit {
    name: String,
    orbits: String,
    count: usize
}

#[derive(Debug, Clone)]
struct OrbitMap {
    orbits: HashMap<String, Orbit>
}

impl OrbitMap {
    pub  fn new(orbits_input: &[&str]) -> Self {
        let mut orbits = HashMap::new();
        for line in orbits_input {
            let l = String::from(*line);
            let mut items = l.split(')');
            let o = String::from(items.next().unwrap());
            let n = String::from(items.next().unwrap());
            let orb = Orbit { name: n.clone(), orbits: o, count: 0 };
            orbits.insert(n, orb);
        }
        OrbitMap {orbits: orbits}
    }

    pub fn total_orbits(&mut self) -> usize {
        let com = self.orbits.get(&String::from("COM"));
        println!("{:?}", com);
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1_part1_should_give_42() {
        let input = vec!["COM)B", "B)C", "C)D", "D)E","E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L" ];
        let mut map = OrbitMap::new(&input);
        println!("{:?}", map);
        assert_eq!(42, map.total_orbits());
    }

    #[test]
    fn sample1_part1_reordered_should_give_42() {
        let input = vec!["B)C", "C)D", "D)E","E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "COM)B" ];
        let mut map = OrbitMap::new(&input);
        assert_eq!(42, map.total_orbits());
    }
}