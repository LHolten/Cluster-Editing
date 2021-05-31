use std::io::{stdin, stdout};

use cluster_lib::{
    disk::{load, write_solution},
    graph::Graph,
    search::search_components,
};

extern crate cluster_lib;

fn main() {
    let mut graph = load(stdin()).unwrap();
    let mut best = Graph::new(0);
    search_components(&mut graph, &mut best);
    write_solution(&graph, &mut best, stdout()).unwrap();
}
