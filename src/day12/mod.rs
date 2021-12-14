mod verify;
use anyhow::{
    anyhow,
    Result,
};
use std::{
    collections::{
        HashMap,
        VecDeque,
    },
    rc::Rc,
    str::FromStr,
};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Cave {
    name: String,
    is_small: bool,
}

impl Cave {
    pub fn new(name: &str) -> Self {
        let is_small = name.chars().all(|c| c.is_lowercase());
        Cave {
            name: name.to_string(),
            is_small,
        }
    }
}

struct CaveConnections {
    map: HashMap<Rc<Cave>, Vec<Rc<Cave>>>,
}

impl CaveConnections {
    fn new() -> Self {
        CaveConnections {
            map: HashMap::new(),
        }
    }

    fn add_connection(&mut self, a: Cave, b: Cave) {
        let a = Rc::new(a);
        let b = Rc::new(b);
        self.map
            .entry(Rc::clone(&a))
            .and_modify(|exits| exits.push(Rc::clone(&b)))
            .or_insert_with(|| vec![Rc::clone(&b)]);
        self.map
            .entry(Rc::clone(&b))
            .and_modify(|exits| exits.push(Rc::clone(&a)))
            .or_insert_with(|| vec![Rc::clone(&a)]);
    }
}

struct CaveSystem {
    connections: CaveConnections,
}

impl FromStr for CaveSystem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut connections = CaveConnections::new();
        for line in s.lines() {
            if let Some((l, r)) = line.trim_end().split_once('-') {
                connections.add_connection(Cave::new(l), Cave::new(r));
            } else {
                return Err(anyhow!("couldn't parse input"));
            }
        }
        Ok(Self { connections })
    }
}

impl CaveSystem {
    fn get_start(&self) -> Option<&Cave> {
        if let Some(rc_cave) = self
            .connections
            .map
            .keys()
            .find(|c| c.name == "start")
        {
            Some(rc_cave)
        } else {
            None
        }
    }

    fn normal_path_filter(cave: &Cave, path: &Path) -> bool {
        !cave.is_small || !path.contains(cave)
    }

    fn advanced_path_filter(cave: &Cave, path: &Path) -> bool {
        !cave.is_small
            || (cave.name != "start"
                && (!path.contains(cave) || !path.visits_small_cave_twice()))
    }

    fn find_all_paths(&self, filter: fn(&Cave, &Path) -> bool) -> Vec<Path> {
        let mut found_paths = vec![];
        let mut queue = VecDeque::new();
        if let Some(start) = self.get_start() {
            queue.push_back((start, Path::new()));
        }

        while let Some((cave, mut path)) = queue.pop_front() {
            path.add(cave);
            if cave.name == "end" {
                found_paths.push(path);
            } else if let Some(connections) = self.connections.map.get(cave) {
                for valid_target in connections
                    .iter()
                    .filter(|cave| filter(cave, &path))
                {
                    queue.push_back((valid_target, path.clone()));
                }
            }
        }
        found_paths
    }
}

#[derive(Clone)]
struct Path<'a> {
    caves: Vec<&'a Cave>,
    small_cave_visited_twice: bool,
}

impl<'a> Path<'a> {
    fn contains(&self, cave: &Cave) -> bool { self.caves.contains(&cave) }

    fn visits_small_cave_twice(&self) -> bool { self.small_cave_visited_twice }

    fn add(&mut self, cave: &'a Cave) {
        if !self.small_cave_visited_twice
            && cave.is_small
            && self.caves.contains(&cave)
        {
            self.small_cave_visited_twice = true;
        }
        self.caves.push(cave);
    }

    fn new() -> Self {
        Path {
            caves: Vec::new(),
            small_cave_visited_twice: false,
        }
    }
}

pub fn day12a(input: &str) -> usize {
    CaveSystem::from_str(input)
        .unwrap()
        .find_all_paths(CaveSystem::normal_path_filter)
        .len()
}

pub fn day12b(input: &str) -> usize {
    CaveSystem::from_str(input)
        .unwrap()
        .find_all_paths(CaveSystem::advanced_path_filter)
        .len()
}
