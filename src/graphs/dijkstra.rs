#[derive(Debug, Clone)]
pub struct Graph {
    adj: Vec<Vec<(usize, u64)>>,
    directed: bool,
}

impl Graph {
    pub fn with_capacity(n: usize, directed: bool) -> Self {
        Self {
            adj: vec![vec![]; n],
            directed,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: u64) {
        self.adj[from].push((to, weight));
        if !self.directed {
            self.adj[to].push((from, weight));
        }
    }

    pub fn neighbors(&self, node: usize) -> &[(usize, u64)] {
        &self.adj[node]
    }

    pub fn node_count(&self) -> usize {
        self.adj.len()
    }
}

pub fn dijkstra(graph: &Graph, start: usize) -> Vec<u64> {
    use std::collections::BinaryHeap;

    #[derive(Clone, Copy, Eq, PartialEq)]
    struct State {
        cost: u64,
        node: usize,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.cost.cmp(&self.cost) // min-heap
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let n = graph.node_count();
    let mut dist = vec![u64::MAX; n];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        node: start,
    });

    while let Some(State { cost, node }) = heap.pop() {
        if cost > dist[node] {
            continue;
        }

        for &(next, weight) in graph.neighbors(node) {
            let next_cost = cost.saturating_add(weight);

            if next_cost < dist[next] {
                dist[next] = next_cost;
                heap.push(State {
                    cost: next_cost,
                    node: next,
                });
            }
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    ///Livro 'Entendendo Algoritmos' página 151
    #[test]
    fn dijkstra_test() {
        let mut g = Graph::with_capacity(6, true);

        g.add_edge(0, 1, 5);
        g.add_edge(0, 2, 0);

        g.add_edge(1, 3, 15);
        g.add_edge(1, 4, 20);

        g.add_edge(2, 3, 30);
        g.add_edge(2, 4, 35);

        g.add_edge(3, 5, 20);
        g.add_edge(4, 5, 10);

        dbg!(&g);

        let dist = dijkstra(&g, 0);
        dbg!(&dist);

        let cost = dist[5];

        assert_eq!(cost, 35);
    }
}
