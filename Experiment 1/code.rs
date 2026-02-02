impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        fn lcm(a: i32, b: i32) -> i32{
            fn gcd(a: i32, b: i32) -> i32{
                if b == 0{
                    a
                }else{
                    gcd(b, a % b)
                }
            }
            (a * b) / gcd(a, b)
        }
        let mut l = lcm(a, b) as i64;
        let mut a = a as i64;
        let mut b = b as i64;
        let mut lo = (a.min(b) - 1) as i64;
        let mut hi = a.min(b) as i64 * n as i64 + 1;
        while hi - lo > 1{
            let m = lo.midpoint(hi);
            let cnt = m / a + m / b - m / l;
            if cnt as i32 >= n{
                hi = m;
            }else{
                lo = m;
            }
        }
        (hi % 1_000_000_007) as i32
    }
}