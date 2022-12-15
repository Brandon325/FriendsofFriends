use std::collections::VecDeque;

//bfs
// returns a vector of tuples, where each tuple contains a node and the nodes that it is connected to.
pub fn breadth_first_search(nodes: Vec<(usize, usize)>) -> Vec<(usize, Vec<usize>)> {
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