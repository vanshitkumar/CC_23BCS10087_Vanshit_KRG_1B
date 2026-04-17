fn shortest_common_supersequence(str1: String, str2: String) -> String {
    let s1 = str1.as_bytes();
    let s2 = str2.as_bytes();
    let m = s1.len();
    let n = s2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if s1[i - 1] == s2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    let mut res = Vec::new();
    let mut i = m;
    let mut j = n;

    while i > 0 && j > 0 {
        if s1[i - 1] == s2[j - 1] {
            res.push(s1[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            res.push(s1[i - 1]);
            i -= 1;
        } else {
            res.push(s2[j - 1]);
            j -= 1;
        }
    }

    while i > 0 {
        res.push(s1[i - 1]);
        i -= 1;
    }
    
    while j > 0 {
        res.push(s2[j - 1]);
        j -= 1;
    }

    res.reverse();
    String::from_utf8(res).unwrap()
}

fn main() {
    let str1 = String::from("abac");
    let str2 = String::from("cab");
    println!("{}", shortest_common_supersequence(str1, str2));

    let str1_2 = String::from("aaaaaaaa");
    let str2_2 = String::from("aaaaaaaa");
    println!("{}", shortest_common_supersequence(str1_2, str2_2));
}
