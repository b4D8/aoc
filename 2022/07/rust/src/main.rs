use std::ops::AddAssign;

use {std::collections::HashMap, utils::Puzzle};

mod parser;

#[cfg(test)]
mod tests;

// Capacity of the filesystem
const DISK_SPACE: usize = 70_000_000;

// Capacity that is required by the update
const NEED_SPACE: usize = 30_000_000;

#[derive(Debug, Clone)]
struct Dir {
    name: String,
}

impl Default for Dir {
    fn default() -> Self {
        Self { name: "/".into() }
    }
}

#[derive(Debug)]
struct Path(Vec<Dir>);

impl Default for Path {
    fn default() -> Self {
        Self(vec![Dir::default()])
    }
}

impl Path {
    // Applies a "change directory" command in place
    fn apply(&mut self, cd: Cd) {
        match cd {
            Cd::Top => *self = Self::default(),
            Cd::In(dir) => self.0.push(dir),
            Cd::Out => {
                self.0.pop();
            }
        }
    }

    // Returns all the paths from root ("/") to leaf
    fn paths(&self) -> Vec<Self> {
        let mut parents = Vec::new();
        for i in 0..self.0.len() {
            let mut path = self.0.clone();
            path.truncate(i + 1);
            parents.push(Self(path));
        }
        parents
    }
}

impl ToString for Path {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .fold(String::new(), |acc, dir| acc + &dir.name)
    }
}

#[derive(Debug, Clone)]
struct File {
    _name: String,
    size: usize,
}

#[derive(Debug, Clone)]
enum Entry {
    File(File),
    Dir(Dir),
}

#[derive(Debug, Clone)]
enum Cmd {
    Ls(Vec<Entry>),
    Cd(Cd),
}

#[derive(Debug, Clone)]
enum Cd {
    In(Dir),
    Out,
    Top,
}

#[derive(Debug, Default, Clone)]
struct Day7 {
    cmd: Vec<Cmd>,
}

#[derive(Debug, Default)]
struct FileSystem {
    dirs: HashMap<String, usize>,
}

impl FileSystem {
    fn new() -> Self {
        let dirs = HashMap::new();
        Self { dirs }
    }

    fn insert(&mut self, path: &Path, entry: Entry) {
        match entry {
            Entry::File(file) => {
                for dir in path.paths() {
                    self.dirs
                        .entry(dir.to_string())
                        .and_modify(|s| s.add_assign(file.size))
                        .or_insert(file.size);
                }
            }
            Entry::Dir(dir) => {
                let dir = [path.to_string(), dir.name].join("");
                self.dirs.insert(dir, 0);
            }
        }
    }

    fn occupied_space(&self, limit: Option<usize>) -> usize {
        if let Some(limit) = limit {
            self.dirs
                .iter()
                .filter_map(|(_, size)| (size <= &limit).then_some(size))
                .sum()
        } else {
            *self.dirs.values().max().unwrap()
        }
    }
}

impl From<Day7> for FileSystem {
    fn from(d: Day7) -> Self {
        let mut fs = Self::new();
        let mut path = Path::default();
        for cmd in d.cmd {
            match cmd {
                Cmd::Cd(cd) => path.apply(cd),
                Cmd::Ls(entries) => entries
                    .into_iter()
                    .for_each(|entry| fs.insert(&path, entry)),
            }
        }
        fs
    }
}

impl Puzzle for Day7 {
    fn from_string(s: String) -> Self {
        s.parse::<Self>().unwrap()
    }

    fn solve1(&self) -> usize {
        FileSystem::from(self.clone()).occupied_space(Some(100_000))
    }

    fn solve2(&self) -> usize {
        let fs: FileSystem = self.clone().into();
        let occupied = fs.occupied_space(None);
        let vacant = DISK_SPACE.checked_sub(occupied).unwrap();
        let needed = NEED_SPACE.checked_sub(vacant).unwrap();
        *fs.dirs
            .values()
            .filter(|size| size > &&needed)
            .reduce(|small, size| if size <= small { size } else { small })
            .unwrap()
    }
}

fn main() {
    let puzzle = Day7::from_file();

    let part1 = puzzle.solve1();
    println!("Part 1: answer is {}.", part1);
    assert_eq!(part1, 1_086_293);

    let part2 = puzzle.solve2();
    println!("Part 2: answer is {}.", part2);
    assert_eq!(part2, 366_028);
}
