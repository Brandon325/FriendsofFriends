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

//Tests
#[test]
fn test_breadth_first_search() {
    let input = vec![(1, 2), (1, 3), (1, 4), (2, 5), (3, 5), (4, 5)];
    let expected_output = vec![(1, vec![2, 3, 4]), (2, vec![5]), (3, vec![5]), (4, vec![5]), (5, vec![])];
    assert_eq!(breadth_first_search(input), expected_output);

    let input = vec![(1, 2), (2, 3), (3, 4), (4, 5), (5, 6)];
    let expected_output = vec![(1, vec![2]), (2, vec![3]), (3, vec![4]), (4, vec![5]), (5, vec![6]), (6, vec![])];
    assert_eq!(breadth_first_search(input), expected_output);

    let input = vec![(1, 2), (1, 3), (1, 4), (1, 5), (1, 6)];
    let expected_output = vec![(1, vec![2, 3, 4, 5, 6]), (2, vec![]), (3, vec![]), (4, vec![]), (5, vec![]), (6, vec![])];
    assert_eq!(breadth_first_search(input), expected_output);
}

#[test]
fn test_find_most_similar_nodes() {
    let input = vec![(1, vec![2, 3, 4]), (2, vec![5]), (3, vec![5]), (4, vec![5]), (5, vec![2,3,4])];
    let expected_output = (1, 5);
    assert_eq!(find_most_similar_nodes(input), expected_output);

    let input = vec![(1, vec![2]), (2, vec![3]), (3, vec![4]), (4, vec![5]), (5, vec![6]), (6, vec![2])];
    let expected_output = (1, 6);
    assert_eq!(find_most_similar_nodes(input), expected_output);

    let input = vec![(1, vec![2, 3, 4, 5, 6]), (2, vec![3,4,5]), (3, vec![]), (4, vec![]), (5, vec![]), (6, vec![])];
    let expected_output = (1, 2);
    assert_eq!(find_most_similar_nodes(input), expected_output);
}
