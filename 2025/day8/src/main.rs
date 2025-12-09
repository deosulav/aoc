use std::{cmp::Ordering, collections::BinaryHeap, fs, mem::swap};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct HeapData {
    dist_sq: i64,
    a: Vec3,
    b: Vec3,
}

impl Ord for HeapData {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist_sq.cmp(&self.dist_sq)
    }
}

impl PartialOrd for HeapData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.dist_sq.cmp(&self.dist_sq))
    }
}

impl PartialEq for HeapData {
    fn eq(&self, other: &Self) -> bool {
        other.dist_sq == self.dist_sq
    }
}

impl Eq for HeapData {}

fn distance_squared(a: &Vec3, b: &Vec3) -> i64 {
    return (a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2);
}

#[derive(Debug)]
struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
    data: Vec<Vec3>,
}

impl DisjointSet {
    fn new() -> DisjointSet {
        DisjointSet {
            parent: Vec::new(),
            size: Vec::new(),
            data: Vec::new(),
        }
    }

    fn insert(&mut self, val: Vec3) -> usize {
        let id = self.parent.len();
        self.data.push(val);
        self.size.push(1);
        self.parent.push(id);
        id
    }

    fn find(&mut self, v: usize) -> usize {
        if v == self.parent[v] {
            return v;
        }
        self.parent[v] = self.find(self.parent[v]);
        self.parent[v]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut ap = self.find(a);
        let mut bp = self.find(b);
        if ap != bp {
            if self.size[ap] < self.size[bp] {
                swap(&mut ap, &mut (bp));
            }
            self.parent[bp] = ap;
            self.size[a] += self.size[b];
        }
    }

    fn get_index(&self, v: &Vec3) -> Option<usize> {
        self.data.iter().position(|a| a == v)
    }

    fn largest_sets(&mut self, k: usize) -> Vec<(usize, usize)> {
        let mut roots = Vec::new();

        for i in 0..self.parent.len() {
            let r = self.find(i);
            if r == i {
                roots.push((i, self.size[i]));
            }
        }

        roots.sort_by_key(|&(_, size)| std::cmp::Reverse(size));

        roots.into_iter().take(k).collect()
    }
}

fn part1() {
    let buffer = fs::read_to_string("sample.txt").unwrap();
    let mut arr: Vec<Vec3> = Vec::new();
    for line in buffer.lines() {
        let mut iter = line.split(",");
        let x = iter.next().unwrap().parse::<i64>().unwrap();
        let y = iter.next().unwrap().parse::<i64>().unwrap();
        let z = iter.next().unwrap().parse::<i64>().unwrap();
        arr.push(Vec3 { x, y, z });
    }

    let mut min_heap: BinaryHeap<HeapData> = BinaryHeap::new();

    let total = arr.len();
    for i in 0..total {
        for j in (i + 1)..total {
            let a = &arr[i];
            let b = &arr[j];
            min_heap.push(HeapData {
                dist_sq: distance_squared(a, b),
                a: a.clone(),
                b: b.clone(),
            })
        }
    }

    // println!("{:#?}", min_heap);

    // Disjoint set union algorithm
    // https://cp-algorithms.com/data_structures/disjoint_set_union.html

    let mut dis = DisjointSet::new();

    'label: for _ in 0..10 {
        let j = min_heap.pop();
        if j.is_none() {
            continue 'label;
        }
        let val = j.unwrap();
        if let None = dis.get_index(&val.a) {
            dis.insert(val.a);
        }

        if let None = dis.get_index(&val.b) {
            dis.insert(val.b);
        }

        dis.union(
            dis.get_index(&val.a).unwrap(),
            dis.get_index(&val.b).unwrap(),
        );
    }

    println!("{:#?}", dis);
    let biggest = dis.largest_sets(3);

    for (root, size) in biggest {
        println!("root {} -> size {}", root, size);
    }

    // println!("Part 1 {}", sum);
}

fn part2() {}

fn main() {
    part1();
    part2();
}
