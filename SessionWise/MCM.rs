fn matrix_chain_order(p: &[usize]) -> usize {
    let n = p.len() - 1; // Number of matrices
    
    let mut m = vec![vec![0; n + 1]; n + 1];

    for l in 2..=n {
        for i in 1..=(n - l + 1) {
            let j = i + l - 1;
            m[i][j] = usize::MAX;

            for k in i..j {
                let q = m[i][k] + m[k + 1][j] + (p[i - 1] * p[k] * p[j]);
                
                if q < m[i][j] {
                    m[i][j] = q;
                }
            }
        }
    }

    m[1][n]
}

fn main() {
    let arr = [10, 30, 5, 60];
    
    let min_mults = matrix_chain_order(&arr);
    println!("Minimum number of multiplications is {}", min_mults);
}