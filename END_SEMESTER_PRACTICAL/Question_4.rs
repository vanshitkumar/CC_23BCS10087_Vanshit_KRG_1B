fn topo_sort_bfs(n: usize, adj: &Vec<Vec<usize>>) -> Vec<usize> {
    use std::collections::VecDeque;
    
    let mut indegree = vec![0; n];

    for u in 0..n {
        for &v in &adj[u] {
            indegree[v] += 1;
        }
    }

    let mut queue = VecDeque::new();

    for i in 0..n {
        if indegree[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut topo = Vec::new();

    while let Some(node) = queue.pop_front() {
        topo.push(node);

        for &neighbor in &adj[node] {
            indegree[neighbor] -= 1;

            if indegree[neighbor] == 0 {
                queue.push_back(neighbor);
            }
        }
    }

    topo
}

fn question_4(){
    let v = 4;
    let edges = [[3, 0], [1, 0], [2, 0]];
    
    let mut adj: Vec<Vec<usize>> = vec![vec![]; 4];
    for edge in edges{
        adj[edge[0]].push(edge[1]);
    }
    
    let ans = topo_sort_bfs(v, &adj);
    
    if ans.len() == v{
        println!("Valid Topo Sort Exists");
    }else{
        println!("Valid Topo Sort Does not Exists");
    }
}

fn main() {
    question_4();
}