// structure representing a graph
// the graph is represented as a list of vertices and a list of edges
// the vertices are represented as a list of structs of type Vertex
// the edges are represented as a list of tuples of type Vertex

use rand::prelude::SliceRandom;
use rand::SeedableRng;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
};

#[derive(Clone, Eq, Hash)]
pub struct Edge<T> {
    pub from: String,
    pub to: String,
    pub weight: T,
}

impl<T> PartialEq for Edge<T> {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
            || self.from == other.to && self.to == other.from
    }
}

impl<T> Edge<T> {
    pub fn new(from: String, to: String, weight: T) -> Self {
        Self { from, to, weight }
    }
}

impl<T> Debug for Edge<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} -> {})", self.from, self.to)
    }
}

#[derive(Debug, Clone)]
pub struct Graph<T> {
    pub vertices: Vec<String>,
    pub edges: Vec<Edge<T>>,
    seed: u64,
}

impl<T> Graph<T>
where
    T: Clone + PartialEq + PartialOrd,
{
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            // init seed with current time
            seed: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn add_node(&mut self, node: String) {
        if self.vertices.contains(&node) {
            panic!("Node already exists");
        }
        self.vertices.push(node);
    }

    pub fn add_edge(&mut self, from: String, to: String, weight: T) {
        let new_edge = Edge::new(from.clone(), to.clone(), weight.clone());
        if self.edges.contains(&new_edge) {
            panic!("Edge already exists");
        }
        self.edges.push(Edge::new(from, to, weight));
    }

    pub fn add_edges(&mut self, edges: Vec<Edge<T>>) {
        for edge in edges {
            if self.edges.contains(&edge) {
                panic!("Edge already exists");
            }
            self.edges.push(edge);
        }
    }

    pub fn is_connected(&self) -> bool {
        let mut visited = HashMap::new();
        let mut stack = Vec::new();

        if self.vertices.is_empty() {
            return false;
        }

        stack.push(&self.vertices[0]);

        while let Some(node) = stack.pop() {
            if visited.contains_key(&node) {
                continue;
            }

            visited.insert(node, true);

            for edge in self.edges.iter() {
                if &edge.from == node {
                    stack.push(&edge.to);
                } else if &edge.to == node {
                    stack.push(&edge.from);
                }
            }
        }

        visited.len() == self.vertices.len()
    }

    pub fn is_cyclic(&self) -> bool {
        let mut visited: HashSet<String> = HashSet::new();
        let mut stack: VecDeque<(String, Option<String>)> = VecDeque::new();

        for vertex in &self.vertices {
            if !visited.contains(vertex) {
                stack.push_back((vertex.clone(), None));

                while let Some((current, parent)) = stack.pop_back() {
                    if visited.contains(&current) {
                        return true; // Cycle detected
                    }

                    visited.insert(current.clone());

                    // Find adjacent vertices
                    for edge in &self.edges {
                        if edge.from == current {
                            if let Some(ref parent) = parent {
                                // Avoid going back to the parent
                                if edge.to == *parent {
                                    continue;
                                }
                            }
                            stack.push_back((edge.to.clone(), Some(current.clone())));
                        }
                        if edge.to == current {
                            if let Some(ref parent) = parent {
                                // Avoid going back to the parent
                                if edge.from == *parent {
                                    continue;
                                }
                            }
                            stack.push_back((edge.from.clone(), Some(current.clone())));
                        }
                    }
                }
            }
        }

        false
    }

    pub fn is_tree(&self) -> bool {
        self.is_connected() && !self.is_cyclic()
    }

    pub fn find_cycle(&self) -> Option<Vec<Edge<T>>> {
        let mut visited: HashSet<String> = HashSet::new();

        if self.vertices.is_empty() {
            return None;
        }

        pub fn dfs<T>(
            graph: &Graph<T>,
            node: String,
            parent: String,
            visited: &mut HashSet<String>,
            path: &mut Vec<Edge<T>>,
        ) -> Option<Vec<Edge<T>>>
        where
            T: Clone,
        {
            visited.insert(node.clone());

            for edge in graph.edges.iter() {
                if edge.from == node || edge.to == node {
                    let next_node = if edge.from == node {
                        edge.to.clone()
                    } else {
                        edge.from.clone()
                    };

                    if next_node == parent {
                        continue;
                    }

                    if !visited.contains(&next_node) {
                        if let Some(mut cycle) = dfs(graph, next_node, node.clone(), visited, path)
                        {
                            cycle.push(edge.clone());
                            return Some(cycle);
                        }
                    }
                }
            }
            None
        }

        return dfs(
            self,
            self.vertices[0].clone(),
            String::new(),
            &mut visited,
            &mut Vec::new(),
        );
    }

    pub fn k_edge_augmentation(
        &mut self,
        k: usize,
        mut edges: Vec<Edge<T>>,
    ) -> Result<(), &'static str> {
        // The k-edge augmentation is a technique used to increase the connectivity of a graph by adding k edges to the graph

        // step 0: check if the graph is already connected or if there is a cycle
        if self.is_connected() {
            return Err("The graph is already connected and cannot be augmented");
        }
        if self.is_cyclic() {
            return Err("The graph contains a cycle and cannot be augmented");
        }

        let mut k = k;
        // seed the random number generator
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed);
        // shuffle the edges
        edges.shuffle(&mut rng);

        // step 1: sort the new edges by their weight
        edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

        // step 2: add the new edges to the graph
        for edge in edges {
            // check if the edge is already in the graph
            if self.edges.contains(&edge) {
                continue;
            }

            self.add_edge(edge.from.clone(), edge.to.clone(), edge.weight.clone());

            // step 3: check if the added edge creates a cycle
            if self.is_cyclic() {
                // step 4: if the added edge creates a cycle, remove it
                self.edges.pop();
            } else {
                // step 5: repeat steps 2 to 4 until k edges have been added
                k -= 1;
                if k == 0 {
                    break;
                }
            }
        }
        if k > 0 {
            return Err("Not enough edges to augment the graph or the graph is already connected");
        }
        Ok(())
    }

    pub fn update_seed(&mut self) {
        self.seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}
