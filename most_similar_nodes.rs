use std::collections::HashMap;
use std::collections::HashSet;

pub fn find_most_similar_nodes(adjacency_list: Vec<(usize, Vec<usize>)>) -> (usize, usize) {
    // Create a map of nodes to their connections
    let mut node_connections: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (node, connections) in adjacency_list {
        let set: HashSet<usize> = connections.into_iter().collect();
        node_connections.insert(node, set);
    }
    // Iterate over the entries in the node_connections map
    let mut most_similar_nodes = (0, 0);
    let mut most_similar_count = 0;
    for (node1, connections1) in &node_connections {
        for (node2, connections2) in &node_connections {
            // Skip the pair if it contains the same node twice
            if node1 == node2 {
                continue;
            }

            // Count the number of connections the two nodes have in common
            let similar_count = connections1.intersection(connections2).count();

            // Update the most similar pair of nodes if necessary
            if similar_count > most_similar_count {
                most_similar_nodes = (*node1, *node2);
                most_similar_count = similar_count;
            }
        }
    }

    most_similar_nodes
}