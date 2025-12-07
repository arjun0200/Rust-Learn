use std::cmp::Ordering;
use std::collections::BinaryHeap;


#[derive(Copy, Clone, Eq, PartialEq)]
struct State{
    cost:usize,
    position:usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge{
    node: usize,
    cost: usize,
}

fn shortest_path(graph: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize>{
    let mut dist :Vec<_> = (0..graph.len()).map(|_| usize::MAX).collect();

    let mut visited = BinaryHeap::new();

    dist[start] = 0;
    visited.push(State {cost: 0, position: start});

    while let Some(State {cost, position}) = visited.pop(){
        if position == goal {return Some(cost);}

        if cost > dist[position]{continue;}

        for edge in & graph[position]{
            let next = State {cost: cost + edge.cost, position: edge.node};

            if next.cost < dist[next.position]{
                visited.push(next);
                dist[next.position] = next.cost;
            }
        }
    } 
    None
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_dijkstra(){
        let graph = vec![
        // node 0
        vec![
            Edge { node: 1, cost: 6 },
            Edge { node: 2, cost: 4 },
            Edge { node: 3, cost: 1 },
        ],

        // node 1
        vec![
            Edge { node: 0, cost: 6 },
            Edge { node: 2, cost: 3 },
        ],

        // node 2
        vec![
            Edge { node: 0, cost: 4 },
            Edge { node: 1, cost: 3 },
            Edge { node: 3, cost: 1 },
        ],

        // node 3
        vec![
            Edge { node: 0, cost: 1 },
            Edge { node: 2, cost: 1 },
        ],
    ];
    assert_eq!(shortest_path(&graph, 0, 1), Some(5));
    }

    

}

// use std::collections::{HashSet, VecDeque};

// #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// pub struct Vertex(u8);

// impl Vertex {
//     pub fn value(self) -> u8 {
//         self.0
//     }

//     pub fn neighbors<'a>(&self, graph: &'a Graph) -> Vec<Vertex> {
//         graph
//             .edges
//             .iter()
//             .filter_map(|(a, b)| {
//                 if *a == self.0 {
//                     Some(Vertex(*b))
//                 } else {
//                     None
//                 }
//             })
//             .collect()
//     }
// }

// #[derive(Clone, Debug)]
// pub struct Graph {
//     vertices: Vec<Vertex>,
//     edges: Vec<(u8, u8)>,
// }

// impl Graph {
//     pub fn new(vertices: Vec<Vertex>, edges: Vec<(u8, u8)>) -> Self {
//         Graph { vertices, edges }
//     }
// }

// pub fn depth_first_search(graph: &Graph, root: Vertex, objective: Vertex) -> Option<Vec<u8>> {
//     let mut visited = HashSet::new();
//     let mut history = Vec::new();

//     // stack for DFS
//     let mut stack = Vec::new();
//     stack.push(root);

//     while let Some(current) = stack.pop() {
//         if !visited.insert(current) {
//             continue;
//         }

//         history.push(current.value());

//         if current == objective {
//             return Some(history);
//         }

//         // reverse neighbors so left-most is processed first
//         let neighbors = current.neighbors(graph);
//         for n in neighbors.into_iter().rev() {
//             if !visited.contains(&n) {
//                 stack.push(n);
//             }
//         }
//     }

//     None
// }

// pub fn breadth_first_search(graph: & Graph, root: Vertex, objective: Vertex) -> Option<Vec<u8>> {
//     let mut visited = HashSet::new();
//     let mut history = Vec::new();

//     // queue for BFS
//     let mut queue = VecDeque::new();
//     queue.push_back(root);

//     while let Some(current) = queue.pop_front() {
//         // if we've already visited this vertex, skip it
//         if !visited.insert(current) {
//             continue;
//         }

//         // record visit order
//         history.push(current.value());

//         // if we found the objective, return the path of visited values
//         if current == objective {
//             return Some(history);
//         }

//         // in BFS we add neighbors to the *back* of the queue
//         let neighbors = current.neighbors(graph);
//         for n in neighbors {
//             if !visited.contains(&n) {
//                 queue.push_back(n);
//             }
//         }
//     }

//     // if we exhaust the queue without finding the objective
//     None
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_dfs() {
//         let vertices = (1..=7).map(Vertex).collect();
//         let edges = vec![(1,2),(1,3),(2,4),(2,5),(3,6),(3,7)];

//         let graph = Graph::new(vertices, edges);

//         let result = depth_first_search(&graph, Vertex(1), Vertex(7));

//         assert_eq!(result, Some(vec![1, 2, 4, 5, 3, 6, 7]));
//     }

//     #[test]
//     fn test_bfs() {
//     let vertices = (1..=7).map(Vertex).collect();
//     let edges = vec![(1,2),(1,3),(2,4),(2,5),(3,6),(3,7)];

//     let graph = Graph::new(vertices, edges);

//     let result = breadth_first_search(&graph, Vertex(1), Vertex(7));

//     // expected BFS order: level by level
//     assert_eq!(result, Some(vec![1, 2, 3, 4, 5, 6, 7]));
// }

// }
