fn question_3(){
    let a = [2, 2, 1];
    let mut cur = 0;
    
    for num in a{
        cur ^= num;
    }
    
    println!("Ans = {}", cur);
}

fn main() {
    question_3();
}