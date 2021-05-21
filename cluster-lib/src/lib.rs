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
        packing::pack,
        search::{search_graph, search_graph_2},
        simplify::simplify,
    };

    #[test]
    fn test() {
        for instance in (1..50).step_by(2) {
            let file_name = format!("../exact/exact{:03}.gr", instance);
            let mut graph = load(File::open(file_name).unwrap()).unwrap();
            // critical(&mut graph);
            println!("{}", search_graph(&mut graph, u32::MAX, &mut 0));
        }
    }

    #[test]
    fn fuzz() {
        let mut graph = load(File::open("../exact/exact003.gr").unwrap()).unwrap();
        loop {
            graph.snapshot();
            simplify(&mut graph, 40);
            search_graph_2(&mut graph, u32::MAX, &mut 0);
            graph.rollback();
        }
    }

    #[test]
    fn lower_bound() {
        let mut bounds = Vec::new();
        for instance in (1..200).step_by(2) {
            let file_name = format!("../exact/exact{:03}.gr", instance);
            let mut graph = load(File::open(file_name).unwrap()).unwrap();
            // critical(&mut graph);
            let lower = pack(&graph);
            let actual = search_graph(&mut graph, u32::MAX, &mut 0);
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

    #[test]
    fn edge_count() {
        let graph = load(File::open("../exact/exact011.gr").unwrap()).unwrap();
        let mut edge_count = Vec::new();
        let mut positive_count = Vec::new();
        for vertex in graph.clusters() {
            edge_count.push(graph.edges(vertex).count());
            positive_count.push(graph.edges(vertex).positive().count());
        }
        let count = edge_count.iter().sum::<usize>() / edge_count.len();
        let positive = positive_count.iter().sum::<usize>() / edge_count.len();
        println!(
            "edge count: {}, positive: {}, vertices: {}",
            count,
            positive,
            edge_count.len() - 1
        )
    }

    #[test]
    fn num_visited() {
        let mut graph = load(File::open("../exact/exact003.gr").unwrap()).unwrap();
        let tests = vec![
            (10, 1000),
            (15, 1000),
            (20, 1000),
            (25, 200),
            (30, 100),
            (35, 50),
            (40, 10),
        ];
        for (num_edges, times) in tests {
            let mut results1 = Vec::new();
            let mut results2 = Vec::new();
            for _ in 0..times {
                graph.snapshot();
                simplify(&mut graph, num_edges);

                graph.snapshot();
                let k = search_graph(&mut graph, u32::MAX, &mut 0);
                graph.rollback();

                graph.snapshot();
                let mut count1 = 0;
                let k1 = search_graph(&mut graph, k, &mut count1);
                results1.push(count1);
                graph.rollback();

                graph.snapshot();
                let mut count2 = 0;
                let k2 = search_graph_2(&mut graph, k, &mut count2);
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
