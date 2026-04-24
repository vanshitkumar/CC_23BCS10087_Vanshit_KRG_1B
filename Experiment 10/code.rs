fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
    let mut bit = vec![0; 20002];
    let mut res = vec![0; nums.len()];
    
    for i in (0..nums.len()).rev() {
        let val = (nums[i] + 10001) as usize;
        
        let mut sum = 0;
        let mut q = val - 1;
        while q > 0 {
            sum += bit[q];
            q -= q & q.wrapping_neg();
        }
        res[i] = sum;
        
        let mut u = val;
        while u < bit.len() {
            bit[u] += 1;
            u += u & u.wrapping_neg();
        }
    }
    
    res
}

fn main() {
    let nums1 = vec![5, 2, 6, 1];
    println!("{:?}", count_smaller(nums1));

    let nums2 = vec![-1];
    println!("{:?}", count_smaller(nums2));

    let nums3 = vec![-1, -1];
    println!("{:?}", count_smaller(nums3));
}
