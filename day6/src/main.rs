use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::str::FromStr;

fn main() -> Result<()> {
    let orbits: Vec<Orbit> = from_file("input.txt")?;
    let map: OrbitsMap = OrbitsMap::from_orbits(orbits);
    println!("Part 1: {}", map.get_orbits_count());
    println!("Part 2: {}", map.get_dist_between("YOU", "SAN"));
    Ok(())
}

#[derive(Debug)]
struct Orbit(String, String);

impl FromStr for Orbit {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self> {
        let split: Vec<&str> = s.split(')').collect();
        Ok(Orbit(split[0].into(), split[1].into()))
    }
}

struct OrbitsMap {
    satellites: HashSet<String>,
    relations: HashMap<String, String>,
}

impl OrbitsMap {
    fn from_orbits(orbits: Vec<Orbit>) -> OrbitsMap {
        let mut satellites = HashSet::new();
        let mut relations = HashMap::new();
        for orbit in orbits {
            satellites.insert(orbit.0.clone());
            satellites.insert(orbit.1.clone());
            relations.insert(orbit.1.clone(), orbit.0.clone());
        }
        OrbitsMap {
            satellites,
            relations,
        }
    }

    fn get_orbits_count(&self) -> usize {
        self.satellites
            .iter()
            .map(|satellite| {
                // println!("Calculating for satellite: {}", satellite);
                self.get_orbit_weight(satellite)
            })
            .sum()
    }

    fn get_orbit_weight(&self, satellite: &String) -> usize {
        // println!("    Getting orbit weight for {}", satellite);
        let weight = match self.relations.get(satellite) {
            None => 0,
            Some(parent) => 1 + self.get_orbit_weight(parent),
        };
        // println!("      weight: {}", weight);
        weight
    }

    fn get_parents(&self, mut satellite: String) -> Vec<String> {
        let mut parents: Vec<String> = Vec::new();
        while let Some(parent) = self.relations.get(&satellite) {
            parents.push(parent.clone());
            satellite = parent.clone();
        }
        parents
    }

    fn get_dist_between(&self, sat1: &str, sat2: &str) -> usize {
        let sat1_parents = self.get_parents(String::from(sat1));
        let sat2_parents = self.get_parents(String::from(sat2));
        let sat1_common_parent_index: usize = sat1_parents
            .iter()
            .position(|parent| sat2_parents.contains(parent))
            .unwrap();
        let sat2_common_parent_index: usize = sat2_parents
            .iter()
            .position(|parent| parent == &sat1_parents[sat1_common_parent_index])
            .unwrap();
        sat1_common_parent_index + sat2_common_parent_index
    }
}

fn from_file(path: &str) -> Result<Vec<Orbit>> {
    let f = File::open(path)?;
    let f = BufReader::new(f).lines();

    let orbits: Vec<Orbit> = f
        .filter_map(|token| token.ok())
        .filter_map(|token| token.parse().ok())
        .collect();

    Ok(orbits)
}
