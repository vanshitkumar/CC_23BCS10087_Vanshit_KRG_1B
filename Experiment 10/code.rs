struct SegTree {
    tree: Vec<i32>,
    n: usize,
}

impl SegTree {
    fn new(n: usize) -> Self {
        SegTree {
            tree: vec![0; 2 * n],
            n,
        }
    }

    fn update(&mut self, mut pos: usize, val: i32) {
        pos += self.n;
        while pos > 0 {
            self.tree[pos] += val;
            pos /= 2;
        }
    }

    fn query(&self, mut l: usize, mut r: usize) -> i32 {
        l += self.n;
        r += self.n;
        let mut sum = 0;
        
        while l < r {
            if l % 2 == 1 {
                sum += self.tree[l];
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                sum += self.tree[r];
            }
            l /= 2;
            r /= 2;
        }
        
        sum
    }
}

fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
    let mut seg_tree = SegTree::new(20005);
    let mut res = vec![0; nums.len()];

    for i in (0..nums.len()).rev() {
        let val = (nums[i] + 10000) as usize;
        res[i] = seg_tree.query(0, val);
        seg_tree.update(val, 1);
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
