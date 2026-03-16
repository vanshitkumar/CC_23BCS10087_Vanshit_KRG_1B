use std::collections::VecDeque;

fn dfs(node: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>, stack: &mut Vec<usize>) {
    visited[node] = true;

    for &neighbor in &adj[node] {
        if !visited[neighbor] {
            dfs(neighbor, adj, visited, stack);
        }
    }

    stack.push(node);
}

fn topo_sort(n: usize, adj: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut visited = vec![false; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        if !visited[i] {
            dfs(i, adj, &mut visited, &mut stack);
        }
    }

    stack.reverse();
    stack
}

fn topo_sort_bfs(n: usize, adj: &Vec<Vec<usize>>) -> Vec<usize> {
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

fn main() {
    let numCourses = 4;
    let prerequisites = [[1,0],[2,0],[3,1],[3,2]];
    
    let mut adj = vec![vec![]; numCourses];
    for [to, from] in prerequisites{
        adj[from].push(to);
    }
    let order = topo_sort(numCourses, &adj);
    for v in order {
        print!("{} ", v);
    }
}