/*
	graph
	This problem requires you to implement a basic graph functio
*/


use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl UndirectedGraph {
    pub fn new() -> Self {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: &str) -> bool {
        if !self.adjacency_table.contains_key(node) {
            self.adjacency_table.insert(node.to_string(), Vec::new());
            true
        } else {
            false
        }
    }

    pub fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (start, end, weight) = edge;
        self.add_node(start);
        self.add_node(end);

        self.adjacency_table.get_mut(start).unwrap().push((end.to_string(), weight));
        self.adjacency_table.get_mut(end).unwrap().push((start.to_string(), weight));
    }

    pub fn contains(&self, node: &str) -> bool {
        self.adjacency_table.contains_key(node)
    }

    pub fn nodes(&self) -> HashSet<String> {
        self.adjacency_table.keys().cloned().collect()
    }

    pub fn edges(&self) -> HashSet<(String, String, i32)> {
        let mut edges = HashSet::new();
        for (from_node, neighbors) in &self.adjacency_table {
            for (to_node, weight) in neighbors {
                let (node1, node2) = if from_node < to_node {
                    (from_node, to_node)
                } else {
                    (to_node, from_node)
                };
                edges.insert((node1.clone(), node2.clone(), *weight));
            }
        }
        edges
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges: HashSet<(String, String, i32)> = [
            ("a".to_string(), "b".to_string(), 5),
            ("b".to_string(), "c".to_string(), 10),
            ("a".to_string(), "c".to_string(), 7),
        ].iter().cloned().collect();

        let edges = graph.edges();
        assert_eq!(edges, expected_edges);
    }
}
