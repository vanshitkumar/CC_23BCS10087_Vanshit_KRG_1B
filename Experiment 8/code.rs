fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
    let n = arr.len();
    let k = k as usize;
    let mut dp = vec![0; n + 1];
    
    for i in 1..=n {
        let mut current_max = 0;
        for j in 1..=k {
            if i >= j {
                current_max = current_max.max(arr[i - j]);
                dp[i] = dp[i].max(dp[i - j] + current_max * j as i32);
            }
        }
    }
    
    dp[n]
}

fn main() {
    let arr1 = vec![1, 15, 7, 9, 2, 5, 10];
    let k1 = 3;
    println!("{}", max_sum_after_partitioning(arr1, k1));

    let arr2 = vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3];
    let k2 = 4;
    println!("{}", max_sum_after_partitioning(arr2, k2));

    let arr3 = vec![1];
    let k3 = 1;
    println!("{}", max_sum_after_partitioning(arr3, k3));
}
