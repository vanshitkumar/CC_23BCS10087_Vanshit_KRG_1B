fn solve(i: &mut I, o: &mut impl std::io::Write) {
    let n: usize = i.n();
    let mut e = [[false; 26]; 26];
    let mut last: Option<Vec<u8>> = None;
    for _ in 0..n{    
        let c: String = i.n();
        let c = c.into_bytes();
        if let Some(l) = &last{
            let ln = l.len();
            let cn = c.len();
            let mut li = 0;
            let mut ci = 0;
            let mut found = false;
            while li < ln && ci < cn{
                if l[li] != c[ci]{
                    let from = (l[li] - b'a') as usize;
                    let to = (c[ci] - b'a') as usize;
                    e[from][to] = true;
                    found = true;
                    break;
                }
                li += 1;
                ci += 1;
            }
            if !found && li < ln{
                writeln!(o, "Impossible").unwrap();
                return;
            }
        }
        last = Some(c);
    }
    
    let mut adj = vec![vec![]; 26];
    for i in 0..26{
        for j in 0..26{
            if e[i][j]{
                adj[i].push(j);
            }
        }
    }
    
    let topo = topo_sort_bfs(26, &adj);
    if topo.len() != 26{
        writeln!(o, "Impossible").unwrap();
    }else{
        for e in topo{
            write!(o, "{}", ((e as u8 + b'a') as char)).unwrap();
        }
    }
}

fn topo_sort_bfs(n: usize, adj: &Vec<Vec<usize>>) -> Vec<usize> {
    use std::collections::VecDeque;
    
    let mut indegree = vec![0; n];

    for u in 0..n {
        for &v in &adj[u] {
            indegree[v] += 1;
        }
    }

    let mut queue = VecDeque::new();

    for i in 0..n {
        if indegree[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut topo = Vec::new();

    while let Some(node) = queue.pop_front() {
        topo.push(node);

        for &neighbor in &adj[node] {
            indegree[neighbor] -= 1;

            if indegree[neighbor] == 0 {
                queue.push_back(neighbor);
            }
        }
    }

    topo
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