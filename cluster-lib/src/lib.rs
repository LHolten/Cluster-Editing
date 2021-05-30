extern crate bit_set;

mod branch;
mod critical;
pub mod disk;
pub mod graph;
pub mod kernel;
mod merge;
mod packing;
pub mod search;
mod simplify;

#[cfg(test)]
mod tests {
    use std::{fs::File, process::Command};

    use crate::{
        disk::{load, write_solution},
        graph::Graph,
        packing::pack,
        search::search_graph,
    };

    #[test]
    fn test() {
        for instance in (1..50).step_by(2) {
            let file_name = format!("../exact/exact{:03}.gr", instance);
            let mut graph = load(File::open(&file_name).unwrap()).unwrap();
            // critical(&mut graph);
            let mut output = Graph::new(1);
            println!(
                "{}",
                search_graph(&mut graph, i32::MAX, &mut 0, &mut output)
            );
            let out_file = format!("../exact/solution{:03}.s", instance);
            write_solution(&graph, &mut output, File::create(&out_file).unwrap()).unwrap();

            assert_eq!(
                Command::new("../verifier/verifier.exe")
                    .args(&[file_name, out_file])
                    .output()
                    .unwrap()
                    .stdout,
                "OK\r\n".bytes().collect::<Vec<_>>()
            )
        }
    }

    // #[test]
    // fn fuzz() {
    //     let mut graph = load(File::open("../exact/exact003.gr").unwrap()).unwrap();
    //     loop {
    //         simplify(&mut graph, 40);
    //         search_graph(&mut graph, u32::MAX, &mut 0, &mut Graph::new(0));
    //     }
    // }

    #[test]
    fn lower_bound() {
        let mut bounds = Vec::new();
        for instance in (1..200).step_by(2) {
            let file_name = format!("../exact/exact{:03}.gr", instance);
            let mut graph = load(File::open(file_name).unwrap()).unwrap();
            // critical(&mut graph);
            let lower = pack(&graph);
            let actual = search_graph(&mut graph, i32::MAX, &mut 0, &mut Graph::new(1));
            println!("{:.1}%", 100. * lower as f32 / actual as f32);
            bounds.push((lower, actual));
        }
        let percentage = bounds
            .iter()
            .map(|(l, a)| *l as f32 / *a as f32)
            .sum::<f32>()
            / bounds.len() as f32;
        println!("{:.1}%", 100. * percentage)

        // let mut graph = load(File::open("../exact/exact007.gr").unwrap()).unwrap();
        // let mut bounds = Vec::new();
        // loop {
        //     graph.snapshot();
        //     simplify(&mut graph, 120);
        //     let lower = pack(&graph);
        //     let actual = search_graph(&mut graph, u32::MAX, &mut 0);
        //     graph.rollback();
        //     bounds.push((lower, actual));
        //     let percentage = bounds
        //         .iter()
        //         .map(|(l, a)| *l as f32 / *a as f32)
        //         .sum::<f32>()
        //         / bounds.len() as f32;
        //     println!("{:.1}%", 100. * percentage)
        // }
    }

    // #[test]
    // fn edge_count() {
    //     let graph = load(File::open("../exact/exact011.gr").unwrap()).unwrap();
    //     let mut edge_count = Vec::new();
    //     let mut positive_count = Vec::new();
    //     for vertex in graph.clusters() {
    //         edge_count.push(graph.edges(vertex).count());
    //         positive_count.push(graph.edges(vertex).positive().count());
    //     }
    //     let count = edge_count.iter().sum::<usize>() / edge_count.len();
    //     let positive = positive_count.iter().sum::<usize>() / edge_count.len();
    //     println!(
    //         "edge count: {}, positive: {}, vertices: {}",
    //         count,
    //         positive,
    //         edge_count.len() - 1
    //     )
    // }

    // #[test]
    // fn kernel() {
    //     for instance in (1..200).step_by(2) {
    //         let file_name = format!("../exact/exact{:03}.gr", instance);
    //         let mut graph = load(File::open(file_name).unwrap()).unwrap();
    //         let mut d = kernel2(&mut graph);
    //         // d += kernelize(&mut graph);
    //         let out_file = format!("../exact/kernel{:03}.gr", instance);
    //         write(&mut graph, File::create(out_file).unwrap()).unwrap();
    //         println!("{}", d);
    //     }
    // }
}
