fn unique_paths(m: usize, n: usize) -> u128 {
    let mut dp = vec![1; n];
    for _ in 1..m {
        for j in 1..n {
            dp[j] += dp[j - 1];
        }
    }
    dp[n - 1]
}

fn main() {
    let m = 3;
    let n = 3;
    
    let paths = unique_paths(m, n);
    
    println!("Unique paths in a {}x{} grid: {}", m, n, paths);
}