use std::collections::BTreeMap;

fn solve(i: &mut I, o: &mut impl std::io::Write) {
    let n: usize = i.n();
    let mut points: Vec<i64> = Vec::new();
    let seg: Vec<(i64, i64)> = (0..n).map(|_|{ 
        let p = (i.n(), i.n());
        points.push(p.0);
        points.push(p.1);
        points.push(p.1 + 1);
        p
    }).collect();
    points.sort_unstable(); points.dedup();
    let mut mp: BTreeMap<i64, usize> = BTreeMap::new();
    let pn = points.len();
    points.iter().enumerate().for_each(|x| _ = mp.insert(*x.1, x.0));
    let mut p = vec![0; pn];
    for seg in seg{
        p[*mp.get(&seg.0).unwrap()] += 1;
        if let Some(v) = p.get_mut(mp.get(&seg.1).unwrap() + 1){
            *v -= 1;
        }
    }
    for i in 1..pn{
        p[i] += p[i-1];
    }
    let mut ans: Vec<i64> = vec![0; n+1];
    assert_eq!(p.len(), points.len());
    let mut cnt = p[0];
    let mut lp = points[0];
    for i in 1..pn{
        ans[cnt as usize] += points[i] - lp;
        lp = points[i];
        cnt = p[i];
    }
    for i in ans.into_iter().skip(1){
        write!(o, "{} ", i).unwrap();
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