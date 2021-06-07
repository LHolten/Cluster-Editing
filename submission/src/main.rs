use std::io::{stdin, stdout};

use cluster_lib::{
    disk::{load, write_solution},
    search::Solver,
};

extern crate cluster_lib;

fn main() {
    let graph = load(stdin()).unwrap();
    let mut solution = Solver::new(graph);
    solution.search_components();
    write_solution(&solution.graph, &mut solution.best, stdout()).unwrap();
}
