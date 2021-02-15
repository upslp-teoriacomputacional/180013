use graphlib::Graph;
use std::io;
use petgraph::Graph;

let mut graph: Graph<usize> = Graph::new();

// Add two vertices to the graph
let id1 = graph.add_vertex('a');
let id2 = graph.add_vertex('b');
let id3 = graph.add_vertex('c');
let id4 = graph.add_vertex('d');
let id5 = graph.add_vertex('e');

// Add an edge between the two vertices
//id1 -> a
//id2 -> b 
//id3 -> c
//id4 -> d
//id5 -> e

graph.add_edge(&id1, &id3);
graph.add_edge(&id2, &id3);
graph.add_edge(&id2, &id5);
graph.add_edge(&id3, &id4);
graph.add_edge(&id3, &id5);
graph.add_edge(&id3, &id1);
graph.add_edge(&id3, &id2);
graph.add_edge(&id5, &id2);
graph.add_edge(&id4, &id3);
graph.add_edge(&id5, &id3);

assert_eq!(*graph.fetch(&id1).unwrap(), 'a');
assert_eq!(*graph.fetch(&id2).unwrap(), 'b');
assert_eq!(*graph.fetch(&id3).unwrap(), 'c');
assert_eq!(*graph.fetch(&id4).unwrap(), 'd');
assert_eq!(*graph.fetch(&id5).unwrap(), 'e');

