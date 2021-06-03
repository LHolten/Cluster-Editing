mod branch;
mod component;
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
    use std::{fs::File, process::Command, time::Instant};

    use crate::{
        disk::{load, write, write_solution},
        graph::Graph,
    };

    #[test]
    fn test() {
        let instances = vec![
            13, 1, 3, 5, 7, 9, 11, 13, 15, 21, 23, 25, 31, 35, 41, 97, 113, 115,
        ];
        for instance in instances {
            let time = Instant::now();
            let file_name = format!("../exact/exact{:03}.gr", instance);
            let mut graph = load(File::open(&file_name).unwrap()).unwrap();
            // critical(&mut graph);
            let mut output = Graph::new(0);
            let count = graph.search_components(&mut output);
            println!("c: {}", count);
            println!("{}", time.elapsed().as_millis());
            let out_file = format!("../exact/solution{:03}.s", instance);
            let out_file2 = format!("../exact/solution{:03}.gr", instance);
            write(&graph, &mut output, File::create(&out_file2).unwrap()).unwrap();

            let count2 =
                write_solution(&graph, &mut output, File::create(&out_file).unwrap()).unwrap();
            assert_eq!(count, count2);

            assert_eq!(
                std::str::from_utf8(
                    &Command::new("../verifier/verifier.exe")
                        .args(&[file_name, out_file])
                        .output()
                        .unwrap()
                        .stdout
                )
                .unwrap(),
                "OK\r\n"
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

    // #[test]
    // fn lower_bound() {
    //     // let mut bounds = Vec::new();
    //     for instance in (1..200).step_by(2) {
    //         let file_name = format!("../exact/exact{:03}.gr", instance);
    //         let mut graph = load(File::open(file_name).unwrap()).unwrap();
    //         // critical(&mut graph);
    //         let lower = pack(&graph);
    //         println!("{}", lower);
    //         // let actual = search_components(&mut graph, i32::MAX, &mut Graph::new(0));
    //         // println!("{:.1}%", 100. * lower as f32 / actual as f32);
    //         // bounds.push((lower, actual));
    //     }
    //     // let percentage = bounds
    //     //     .iter()
    //     //     .map(|(l, a)| *l as f32 / *a as f32)
    //     //     .sum::<f32>()
    //     //     / bounds.len() as f32;
    //     // println!("{:.1}%", 100. * percentage)
    // }

    #[test]
    fn edge_count() {
        let graph = load(File::open("../exact/exact025.gr").unwrap()).unwrap();
        for vertex in graph.clusters.clone() {
            println!("{}", graph.positive(vertex, 0).count());
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
