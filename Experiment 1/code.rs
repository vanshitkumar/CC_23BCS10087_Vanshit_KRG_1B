struct Solution;

impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        let mut size = nums.len();
        while size != 1 {
            for i in 0..size-1 {
                nums[i] = (nums[i] + nums[i + 1]) % 10;
            }
            size -= 1;
        }
        nums[0]
    }
}