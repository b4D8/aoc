use {std::collections::HashMap, std::ops::AddAssign, utils::Puzzle};

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
    fn go_to(&mut self, dest: Dest) {
        match dest {
            Dest::Root => *self = Self::default(),
            Dest::Forward(dir) => self.0.push(dir),
            Dest::Backward => {
                self.0.pop();
            }
        }
    }

    // Returns all the paths from root ("/") to leaf directory
    fn paths(&self) -> Vec<Self> {
        (0..self.0.len())
            .map(|i| {
                let mut path = self.0.clone();
                path.truncate(i + 1);
                Self(path)
            })
            .collect()
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
enum Command {
    List(Vec<Entry>),
    Change(Dest),
}

#[derive(Debug, Clone)]
enum Dest {
    Root,
    Forward(Dir),
    Backward,
}

#[derive(Debug, Default, Clone)]
struct Day7 {
    cmd: Vec<Command>,
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

    fn canonicalize(path: &Path, dir: Dir) -> String {
        [path.to_string(), dir.name].join("")
    }

    fn insert(&mut self, path: &Path, entry: Entry) {
        match entry {
            Entry::Dir(dir) => {
                self.dirs.insert(Self::canonicalize(path, dir), 0);
            }
            Entry::File(file) => path.paths().into_iter().for_each(|dir| {
                self.dirs
                    .entry(dir.to_string())
                    .and_modify(|s| s.add_assign(file.size))
                    .or_insert(file.size);
            }),
        }
    }

    fn occupied(&self, limit: Option<usize>) -> usize {
        let dir_space = self.dirs.values();
        if let Some(limit) = limit {
            dir_space
                .filter_map(|size| (size <= &limit).then_some(size))
                .sum()
        } else {
            *dir_space.max().unwrap()
        }
    }
}

impl From<Day7> for FileSystem {
    fn from(day: Day7) -> Self {
        let mut fs = Self::new();
        let mut path = Path::default();
        day.cmd.into_iter().for_each(|cmd| match cmd {
            Command::Change(dest) => path.go_to(dest),
            Command::List(entries) => entries
                .into_iter()
                .for_each(|entry| fs.insert(&path, entry)),
        });
        fs
    }
}

impl Puzzle for Day7 {
    fn from_string(s: String) -> Self {
        s.parse::<Self>().unwrap()
    }

    fn solve1(&self) -> usize {
        FileSystem::from(self.clone()).occupied(Some(100_000))
    }

    fn solve2(&self) -> usize {
        let fs: FileSystem = self.clone().into();
        let occupied = fs.occupied(None);
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
