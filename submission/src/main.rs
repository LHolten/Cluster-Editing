use std::io::{stdin, stdout};

use cluster_lib::{
    disk::{load, write_solution},
    graph::Graph,
    search::search_graph,
};

extern crate cluster_lib;

fn main() {
    let mut graph = load(stdin()).unwrap();
    let mut best = Graph::new(1);
    search_graph(&mut graph, i32::MAX, &mut best);
    write_solution(&graph, &mut best, stdout()).unwrap()
}
