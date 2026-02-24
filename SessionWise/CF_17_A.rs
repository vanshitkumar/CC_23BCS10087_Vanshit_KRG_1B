fn solve(i: &mut I, o: &mut impl std::io::Write) {
    let n: usize = i.n();
    let k: usize = i.n();
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut primes = Vec::new();
    for i in 2..=n{
        if is_prime[i]{
            primes.push(i);
            let mut j = i * i;
            while j <= n{
                is_prime[j] = false;
                j += i;
            }
        }
    }
    let cnt = primes.windows(2).fold(0usize, |s, w|{
        s + (w[0] + w[1] + 1 <= n && is_prime[w[0] + w[1] + 1]) as usize
    });
    writeln!(o, "{}", if cnt >= k{"YES"} else{"NO"}).unwrap();
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