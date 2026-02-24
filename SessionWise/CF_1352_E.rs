use std::io::{self, Read, Write};

fn solve(i: &mut I, o: &mut impl Write) {
    let n: usize = i.n();
    let v: Vec<usize> = (0..n).map(|_| i.n()).collect();
    let mut special = vec![false; n + 1];
    for i in 0..n{
        let mut sm = v[i];
        for j in (i+1)..n{
            sm += v[j];
            if sm <= n{
                special[sm] = true;
            }
        }
    }
    let ans = v.into_iter().fold(0, |x, n| {if special[n] {x + 1} else {x}});
    writeln!(o, "{}", ans).unwrap();
}

fn main() {
    let mut b = String::new();
    io::stdin().read_to_string(&mut b).unwrap();
    let mut i = I::new(&b);
    let o = io::stdout();
    let mut out = io::BufWriter::new(o.lock());
	let t: usize = i.n();
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