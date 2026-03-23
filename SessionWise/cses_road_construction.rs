pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
    mx: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|i| i).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            size: n,
            mx: 1
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            let px = self.parent[x];
            self.parent[x] = self.find(px);
            self.parent[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let parent_x = self.find(x);
        let parent_y = self.find(y);
        if parent_x == parent_y {
            return false;
        }

        let (large, small) = if self.sizes[parent_x] < self.sizes[parent_y] {
            (parent_y, parent_x)
        } else {
            (parent_x, parent_y)
        };

        self.parent[small] = large;
        self.sizes[large] += self.sizes[small];
        self.sizes[small] = 0;
        self.size -= 1;
        self.mx = self.mx.max(self.sizes[large]);
        true
    }
}

fn solve(i: &mut I, o: &mut impl std::io::Write) {
    let n: usize = i.n();
    let m: usize = i.n();
    
    let mut dsu = UnionFind::new(n);
    
    for _ in 0..m{
        let from: usize = i.n();
        let to: usize = i.n();
        dsu.unite(from - 1, to - 1);
        writeln!(o, "{} {}", dsu.size, dsu.mx).unwrap();
    }
}

fn main() {
    let mut b = String::new();
    use std::io::Read;
    std::io::stdin().read_to_string(&mut b).unwrap();
    let mut i = I::new(&b);
    let o = std::io::stdout();
    let mut out = std::io::BufWriter::new(o.lock());
	let t: usize = 1;
    for _ in 0..t {
		solve(&mut i, &mut out)
	};
}
struct I<'a> {
    it: std::str::SplitWhitespace<'a>,
}
impl<'a> I<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            it: s.split_whitespace(),
        }
    }
    fn n<T: std::str::FromStr>(&mut self) -> T {
        self.it.next().unwrap().parse().ok().unwrap()
    }
}