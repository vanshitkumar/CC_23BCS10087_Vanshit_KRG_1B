fn min_energy(mut tasks: Vec<(i64, i64)>) -> i64{
    tasks.sort_by(|a, b|{
        (a.1 - a.0).cmp(&(b.1 - b.0)).reverse()
    });
    let mut cur = 0;
    let mut ans = 0;
    for task in tasks{
        if cur < task.1{
            ans += task.1 - cur;
            cur = task.1;
        }
        cur -= task.0
    }
    ans
}

fn main(){
    let tasks = vec![(1,2), (2, 4), (4, 8)];
    println!("Minimum Energy: {}", min_energy(tasks));
}