use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::collections::HashSet;

fn read_file(path: &str) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = v[0].parse::<usize>().unwrap();
        let y = v[1].parse::<usize>().unwrap();
        result.push((x, y));
    }
    return result;
}

//bfs
// returns a vector of tuples, where each tuple contains a node and the nodes that it is connected to.
fn breadth_first_search(nodes: Vec<(usize, usize)>) -> Vec<(usize, Vec<usize>)> {
    // Create a map of nodes to the nodes they are connected to
    let mut adjacency_list: Vec<(usize, Vec<usize>)> = Vec::new();
    for (node1, node2) in nodes {
        // Add node1 to the list if it does not already exist
        if !adjacency_list.iter().any(|(n, _)| *n == node1) {
            adjacency_list.push((node1, Vec::new()));
        }

        // Add node2 to the list if it does not already exist
        if !adjacency_list.iter().any(|(n, _)| *n == node2) {
            adjacency_list.push((node2, Vec::new()));
        }

        // Add an edge from node1 to node2
        adjacency_list
            .iter_mut()
            .find(|(n, _)| *n == node1)
            .unwrap()
            .1
            .push(node2);
    }

    // Use a queue to implement breadth first search
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut visited: Vec<usize> = Vec::new();

    // Start the search at the first node in the adjacency list
    queue.push_back(adjacency_list[0].0);

    // Continue searching until the queue is empty
    while !queue.is_empty() {
        // Get the next node in the queue
        let node = queue.pop_front().unwrap();

        // Skip this node if it has already been visited
        if visited.contains(&node) {
            continue;
        }

        // Mark this node as visited and add it to the list of visited nodes
        visited.push(node);

        // Add all of the unvisited neighbors of this node to the queue
        let neighbors = adjacency_list
            .iter()
            .find(|(n, _)| *n == node)
            .unwrap()
            .1
            .clone();
        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                queue.push_back(neighbor);
            }
        }
    }

    // Return the list of nodes and their connections
    adjacency_list
} 

fn find_least_similar_nodes(adjacency_list: Vec<(usize, Vec<usize>)>) -> (usize, usize) {
    // Create a map of nodes to their connections
    let mut node_connections: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (node, connections) in adjacency_list {
        let set: HashSet<usize> = connections.into_iter().collect();
        node_connections.insert(node, set);
    }

    // Iterate over the entries in the node_connections map
    let mut least_similar_nodes = (0, 0);
    let mut least_similar_count = std::usize::MAX;
    for (node1, connections1) in &node_connections {
        for (node2, connections2) in &node_connections {
            // Skip the pair if it contains the same node twice
            if node1 == node2 {
                continue;
            }

            // Count the number of connections the two nodes have in common
            let similar_count = connections1.intersection(connections2).count();

            // Update the least similar pair of nodes if necessary
            if similar_count < least_similar_count {
                least_similar_nodes = (*node1, *node2);
                least_similar_count = similar_count;
            }
        }
    }

    least_similar_nodes
}

fn find_most_similar_nodes(adjacency_list: Vec<(usize, Vec<usize>)>) -> (usize, usize) {
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


fn main() {
    let nodes = read_file("facebook_combined.txt");
    let bfs = breadth_first_search(nodes);
    let bfs2 = bfs.clone();
    //println!("Nodes {:?} and have the most similar connections.", bfs);
    let similarbfs = find_most_similar_nodes(bfs);
    println!("Nodes {:?} and have the most similar connections.", similarbfs);
    let leastbfs = find_least_similar_nodes(bfs2);
    println!("Nodes {:?} and have the least similar connections.", leastbfs);
}
