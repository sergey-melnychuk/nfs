use std::collections::{HashSet, VecDeque};

/// Adjacency-list representation of a weighted bidirectional graph
pub struct Graph {
    nodes: usize,
    edges: Vec<Vec<(usize, usize)>>,
}

impl Graph {
    /// Create a new graph of a given size (node count)
    pub fn new(nodes: usize) -> Self {
        Self {
            nodes,
            edges: vec![vec![]; nodes],
        }
    }

    /// Return the size of the graph (node count)
    pub fn len(&self) -> usize {
        self.nodes
    }

    pub fn is_empty(&self) -> bool {
        // clippy complains when `len` goes without `is_empty`
        self.nodes == 0
    }

    /// Add weighted bi-directional edge between the `src` and `dst` nodes
    pub fn link(&mut self, src: usize, dst: usize, weight: usize) {
        assert!(src < self.nodes && dst < self.nodes, "invalid edge");
        self.edges[src].push((dst, weight));
        self.edges[dst].push((src, weight));
    }

    /// Return edges going from the provided node and respective weights
    pub fn peers(&self, src: usize) -> &[(usize, usize)] {
        assert!(src < self.nodes, "invalid node");
        &self.edges[src]
    }
}

fn bfs<T>(graph: &Graph, src: usize, mut acc: T, mut hit: impl FnMut(&mut T, (usize, usize))) -> T {
    let mut seen: HashSet<usize> = HashSet::with_capacity(graph.len());
    seen.insert(src);

    let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();
    frontier.push_back((src, 0));

    while let Some((node, cost)) = frontier.pop_front() {
        seen.insert(node);
        hit(&mut acc, (node, cost));
        for (next, step) in graph.peers(node) {
            let cost = cost + step;
            hit(&mut acc, (node, cost));
            if !seen.contains(next) {
                frontier.push_back((*next, cost));
            }
        }
    }

    acc
}

/// Run Dijkstra's shortest path algorithm from the given node,
/// Returns list of distances to the nodes:
/// - `0` stands for the source node,
/// - `usize::MAX` stands for unreachable node.
fn dijkstra(graph: &Graph, src: usize) -> Vec<usize> {
    let acc = vec![usize::MAX; graph.len()];
    bfs(graph, src, acc, |acc, (node, cost)| {
        acc[node] = acc[node].min(cost);
    })
}

/// Run network flow simulation, return for each node:
/// - total time to reach the nodes (only for reachable nodes)
/// - total number of nodes reached (excluding the source one)
pub fn flow(graph: &Graph) -> Vec<(usize, usize)> {
    (0..graph.len())
        .map(|src| {
            let costs = dijkstra(graph, src)
                .into_iter()
                .filter(|cost| cost < &usize::MAX && cost > &0)
                .collect::<Vec<_>>();
            let time = costs.iter().max().cloned().unwrap_or_default();
            let nodes = costs.len();
            (time, nodes)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut graph = Graph::new(3);
        graph.link(0, 1, 200);
        graph.link(1, 2, 350);
        graph.link(0, 2, 500);
        graph.link(1, 2, 600);

        assert_eq!(dijkstra(&graph, 0), vec![0, 200, 500]);
        assert_eq!(dijkstra(&graph, 1), vec![200, 0, 350]);
        assert_eq!(dijkstra(&graph, 2), vec![500, 350, 0]);
    }

    #[test]
    fn test_simple() {
        /*
             300
          _________
         /         \
        0 --- 1 --- 2
          100   100

        */
        let mut graph = Graph::new(3);
        graph.link(0, 1, 100);
        graph.link(1, 2, 100);
        graph.link(0, 2, 300);

        assert_eq!(dijkstra(&graph, 0), vec![0, 100, 200],)
    }
}
