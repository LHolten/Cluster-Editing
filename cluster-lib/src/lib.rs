mod branch;
mod critical;
pub mod disk;
mod graph;
pub mod kernel;
mod merge;
mod packing;
pub mod search;
mod simplify;

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::{
        critical::critical,
        disk::{load, write},
        kernel::{kernel2, kernelize},
        search::{search_graph, search_graph_2},
        simplify::simplify,
    };

    // #[test]
    // fn test() {
    //     for instance in (1..50).step_by(2) {
    //         let file_name = format!("../exact/exact{:03}.gr", instance);
    //         let mut graph = load(File::open(file_name).unwrap()).unwrap();
    //         // critical(&mut graph);
    //         println!("{}", search_graph_2(&mut graph, u32::MAX));
    //     }
    // }

    #[test]
    fn num_visited() {
        let mut graph = load(File::open("../exact/exact003.gr").unwrap()).unwrap();
        for num_edges in (10..=50).step_by(10) {
            let mut results1 = Vec::new();
            let mut results2 = Vec::new();
            for _ in 0..50 {
                graph.snapshot();
                simplify(&mut graph, num_edges);

                graph.snapshot();
                let mut count1 = 0;
                let k1 = search_graph(&mut graph, i32::MAX as u32, &mut count1);
                results1.push(count1);
                graph.rollback();

                graph.snapshot();
                let mut count2 = 0;
                let k2 = search_graph_2(&mut graph, k1, &mut count2);
                results2.push(count2);
                graph.rollback();

                if k1 != k2 {
                    write(&mut graph, File::create("test.gr").unwrap()).unwrap();
                }
                assert_eq!(k1, k2);
                graph.rollback();
            }
            println!(
                "1 had avg {} on {}",
                results1.iter().sum::<usize>() / results1.len(),
                num_edges
            );
            println!(
                "2 had avg {} on {}",
                results2.iter().sum::<usize>() / results2.len(),
                num_edges
            );
        }
    }

    // #[test]
    // fn test_kernelize() {
    //     for instance in (1..200).step_by(2) {
    //         let file_name = format!("../exact/exact{:03}.gr", instance);
    //         let mut graph = load(File::open(file_name).unwrap()).unwrap();
    //         let mut d = kernel2(&mut graph);
    //         d += kernelize(&mut graph);
    //         let out_file = format!("../exact/kernel{:03}.gr", instance);
    //         write(&mut graph, File::create(out_file).unwrap()).unwrap();
    //         println!("{}", d);
    //     }
    // }
}
