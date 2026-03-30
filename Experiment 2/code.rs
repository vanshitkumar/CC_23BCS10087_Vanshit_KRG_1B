use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        
        
        if n <= 2 {
            return n as i32;
        }

        let mut globalMax = 0;

        for i in 0..n - 1 {
            let mut map: HashMap<(i32, i32), i32> = HashMap::new();
            let mut localMax = 0;

            for j in i + 1..n {
                let mut dx = points[j][0] - points[i][0];
                let mut dy = points[j][1] - points[i][1];

                let hcf = Self::hcf(dx, dy);

                dx /= hcf;
                dy /= hcf;

                let slope = (dx, dy);

                
                let count = map.entry(slope).or_insert(0);
                *count += 1;

                localMax = cmp::max(localMax, *count);
            }
            globalMax = cmp::max(globalMax, localMax + 1);
        }

        globalMax
    }

    fn hcf(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}