use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

trait Size {
    fn size(&self) -> i32;
}

#[derive(Debug)]
struct Dir {
    pub files: HashMap<String, File>,
    pub dirs: HashMap<String, Dir>,
}

impl Dir {
    fn new() -> Dir {
        Dir {
            files: HashMap::new(),
            dirs: HashMap::new(),
        }
    }

    fn insert_file(&mut self, name: &str, file: File) {
        self.files.entry(name.to_string()).or_insert(file);
    }

    fn insert_dir(&mut self, name: &str, dir: Dir) {
        self.dirs.entry(name.to_string()).or_insert(dir);
    }

    fn get_dir(&mut self, name: &str) -> Option<&mut Dir> {
        self.dirs.get_mut(&name.to_string())
    }

    fn get_path(&mut self, path: Vec<&&str>) -> Option<&mut Dir> {
        let mut d = Some(self);
        for p in path.iter() {
            d = d.unwrap().get_dir(p);
            d.as_ref()?;
        }
        d
    }
}

impl Size for Dir {
    fn size(&self) -> i32 {
        self.files.values().map(|t| t.size()).sum::<i32>()
            + self.dirs.values().map(|t| t.size()).sum::<i32>()
    }
}

#[derive(Debug)]
struct File {
    pub size: i32,
}

impl File {
    fn new(size: i32) -> File {
        File { size }
    }
}

impl Size for File {
    fn size(&self) -> i32 {
        self.size
    }
}

fn part1_sum(dir: &Dir) -> i32 {
    let mut n = 0;
    for d in dir.dirs.values() {
        if d.size() <= 100000 {
            n += d.size();
        }
        n += part1_sum(d);
    }
    n
}

fn part2_find(dir: &Dir, free: i32) -> Option<i32> {
    let mut n = None;
    for d in dir.dirs.values() {
        if d.size() + free >= 30000000 && (n.is_none() || d.size() < n.unwrap()) {
            n = Some(d.size())
        }
        if let Some(dn) = part2_find(d, free) {
            if n.is_none() || dn < n.unwrap() {
                n = Some(dn);
            }
        }
    }
    n
}

fn main() {
    let input = fs::read_to_string("./inputs/07.txt").unwrap();

    let mut root = Dir::new();
    let mut path: VecDeque<&str> = VecDeque::new();
    let mut log = input.split('\n').filter(|t| !t.is_empty());

    let mut l = log.next();
    while l.is_some() {
        let p = l.unwrap().split(' ').collect::<Vec<_>>();

        if p[1] == "cd" {
            if p[2] == "/" {
                path.clear();
            } else if p[2] == ".." {
                path.pop_back();
            } else {
                path.push_back(p[2]);
            }
            l = log.next();
        } else if p[1] == "ls" {
            l = log.next();
            while l.is_some() && !l.unwrap().starts_with('$') {
                let p = l.unwrap().split(' ').collect::<Vec<_>>();
                if p[0] == "dir" {
                    root.get_path(path.iter().collect())
                        .unwrap()
                        .insert_dir(p[1], Dir::new());
                } else {
                    let size = p[0].parse::<i32>().unwrap();
                    root.get_path(path.iter().collect())
                        .unwrap()
                        .insert_file(p[1], File::new(size))
                }
                l = log.next();
            }
        } else {
            return;
        }
    }

    println!("part1 {:?}", part1_sum(&root));
    println!("part2 {:?}", part2_find(&root, 70000000 - root.size()))
}
