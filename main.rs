mod bfs;
mod file_reader;
mod least_similar_nodes;
mod most_similar_nodes;

use crate::file_reader::read_file;
use crate::bfs::breadth_first_search;
use crate::least_similar_nodes::find_least_similar_nodes;
use crate::most_similar_nodes::find_most_similar_nodes;


fn main() {
    // Read the input file
    let input = read_file("facebook_combined.txt");

    // Use BFS to create an adjacency list
    let adjacency_list = breadth_first_search(input);
    let adjacency_list2 = adjacency_list.clone();
    // Find the least similar pair of nodes
    let least_similar_nodes = find_least_similar_nodes(adjacency_list);
    println!("The least similar nodes are: {:?}", least_similar_nodes);

    // Find the most similar pair of nodes
    let most_similar_nodes = find_most_similar_nodes(adjacency_list2);
    println!("The most similar nodes are: {:?}", most_similar_nodes);
}