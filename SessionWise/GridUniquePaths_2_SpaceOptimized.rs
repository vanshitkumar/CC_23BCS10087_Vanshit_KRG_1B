fn unique_paths_with_obstacles_optimized(grid: Vec<Vec<i32>>) -> u128 {
    let n = grid[0].len();
    // Use a single row to store results
    let mut dp = vec![0; n];

    // Starting point
    if grid[0][0] == 0 { dp[0] = 1; }

    for row in grid {
        for j in 0..n {
            if row[j] == 1 {
                dp[j] = 0; // Obstacle blocks all paths
            } else if j > 0 {
                // dp[j] is the value from the row above
                // dp[j-1] is the value from the left
                dp[j] += dp[j - 1];
            }
        }
    }
    dp[n - 1]
}