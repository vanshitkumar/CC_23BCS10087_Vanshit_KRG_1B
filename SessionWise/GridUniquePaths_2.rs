fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> u128 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();

    // If the starting point or ending point is an obstacle, no paths exist
    if obstacle_grid[0][0] == 1 || obstacle_grid[m - 1][n - 1] == 1 {
        return 0;
    }

    let mut dp = vec![0; n];

    // Base case: Starting point
    dp[0] = 1;

    for i in 0..m {
        for j in 0..n {
            if obstacle_grid[i][j] == 1 {
                // If there's an obstacle, 0 ways to reach/pass through this cell
                dp[j] = 0;
            } else if j > 0 {
                // Number of ways = ways from above (current dp[j]) 
                // + ways from the left (dp[j-1])
                dp[j] += dp[j - 1];
            }
        }
    }

    dp[n - 1]
}

fn main() {
    // Example: 3x3 grid with an obstacle in the center
    // 0 = Empty, 1 = Obstacle
    let grid = vec![
        vec![0, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 0],
    ];

    let result = unique_paths_with_obstacles(grid);
    println!("Unique paths with obstacles: {}", result);
}