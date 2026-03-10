fn max_product(words: Vec<&str>) -> i64{
    let n = words.len();
    let mut ans = 0;
    let mut hash = vec![0; n];
    for (i, word) in words.iter().enumerate(){
        for c in word.bytes(){
            hash[i] |= 1 << (c - b'a');
        }
    }
    for i in 0..n{
        for j in i+1..n{
            if hash[i] & hash[j] == 0{
                ans = ans.max(words[i].len() * words[j].len());
            }
        }
    }
    ans as i64
}

fn main(){
    let words = vec!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"];
    println!("Max Product: {}", max_product(words));
}