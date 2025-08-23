//785. Is Graph Bipartite?

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut color = vec![0; n]; 
        fn dfs(node: usize, c: i32, graph: &Vec<Vec<i32>>, color: &mut Vec<i32>) -> bool {
            if color[node] != 0 {
                return color[node] == c;
            }
            color[node] = c;
            for &neighbor in &graph[node] {
                let neighbor = neighbor as usize;
                if !dfs(neighbor, -c, graph, color) {
                    return false;
                }
            }
            true
        }

        for i in 0..n {
            if color[i] == 0 && !dfs(i, 1, &graph, &mut color) {
                return false;
            }
        }

        true
    }
}
