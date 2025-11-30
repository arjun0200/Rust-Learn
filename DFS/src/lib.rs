
use std::collections::{HashSet, VecDeque};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Vertex(u8);

impl Vertex {
    pub fn value(self) -> u8 {
        self.0
    }

    pub fn neighbors<'a>(&self, graph: &'a Graph) -> Vec<Vertex> {
        graph
            .edges
            .iter()
            .filter_map(|(a, b)| {
                if *a == self.0 {
                    Some(Vertex(*b))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<(u8, u8)>,
}

impl Graph {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<(u8, u8)>) -> Self {
        Graph { vertices, edges }
    }
}

pub fn depth_first_search(graph: &Graph, root: Vertex, objective: Vertex) -> Option<Vec<u8>> {
    let mut visited = HashSet::new();
    let mut history = Vec::new();

    // stack for DFS
    let mut stack = Vec::new();
    stack.push(root);

    while let Some(current) = stack.pop() {
        if !visited.insert(current) {
            continue;
        }

        history.push(current.value());

        if current == objective {
            return Some(history);
        }

        // reverse neighbors so left-most is processed first
        let neighbors = current.neighbors(graph);
        for n in neighbors.into_iter().rev() {
            if !visited.contains(&n) {
                stack.push(n);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs() {
        let vertices = (1..=7).map(Vertex).collect();
        let edges = vec![(1,2),(1,3),(2,4),(2,5),(3,6),(3,7)];

        let graph = Graph::new(vertices, edges);

        let result = depth_first_search(&graph, Vertex(1), Vertex(7));

        assert_eq!(result, Some(vec![1, 2, 4, 5, 3, 6, 7]));
    }
}
