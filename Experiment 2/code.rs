impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut f = vec![0i32; 31];
        let n = nums.len() as i32;
        for num in nums {
            for i in 0..30 {
                if (num >> i) & 1 == 1 {
                    f[i] += 1;
                }
            }
        }
        f.iter().map(|&x| x * (n - x)).sum()
    }
}
