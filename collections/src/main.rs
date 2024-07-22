use std::collections::HashMap;
fn dfs(i: i32, graph: &[Vec<i32>; 5]) {
    println!("{}", i);
    let k = i as usize;
    for j in &graph[k] {
        dfs(*j, &graph);
    }
}
#[derive(Debug)]
struct DSU {
    parents: Vec<i32>,
    size: Vec<i32>,
}

impl DSU {
    fn new(size: i32) -> DSU {
        let parents: Vec<i32> = (0..=size).collect();
        let size = vec![1; size as usize + 1];
        DSU { parents, size }
    }
    fn union(&mut self, x: i32, y: i32) {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if self.size[x as usize] > self.size[y as usize] {
            std::mem::swap(&mut x, &mut y);
        }
        if x != y {
            self.parents[y as usize] = x;
            self.size[x as usize] += self.size[y as usize];
        }
    }
    fn find(&mut self, i: i32) -> i32 {
        if self.parents[i as usize] == i {
            return i;
        }
        self.parents[i as usize] = self.find(self.parents[i as usize]);
        return self.parents[i as usize];
    }
}
fn main() {
    let mut arr: [Vec<i32>; 5] = Default::default();
    let mut graph = vec![String::from("Hello"); 6];
    let mut mp = HashMap::new();
    mp.insert(0, 1);
    let a: &mut i32 = mp.entry(0).or_insert(0);
    *a += 1;
    println!("{:?}", a);
    let b = mp.get(&0);

    println!("{:?}", b);
    let b = mp.get(&0);
    arr[0].push(1);
    arr[2].push(1);
    arr[3].push(1);
    arr[1].push(4);
    graph[0].push_str("from the other side");
    for s in &graph {
        println!("{}", s);
    }
    println!("{:?}", graph);
    dfs(0, &arr);
    let mut dsu = DSU::new(10);
    dsu.union(1, 2);
    let p = dsu.find(2);
    println!("The parent of 2 is {}", p);
    println!("{:?} {:?}", dsu.parents, dsu.size);
}
