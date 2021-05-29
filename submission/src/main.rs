use std::io::{stdin, stdout};

use cluster_lib::{
    disk::{load, write_solution},
    graph::Graph,
    search::search_graph,
};

extern crate cluster_lib;

fn main() {
    let mut graph = load(stdin()).unwrap();
    graph.snapshot();
    let mut best = Graph::new(0);
    search_graph(&mut graph, u32::MAX, &mut 0, &mut best);
    graph.rollback();
    write_solution(&graph, &mut best, stdout()).unwrap()
}
