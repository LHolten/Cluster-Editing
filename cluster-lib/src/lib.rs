mod branch;
mod component;
pub mod disk;
pub mod graph;
mod matrix;
mod merge;
mod packing;
pub mod search;
mod triple;

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{self, Write},
        process::Command,
        time::Instant,
    };

    use crate::{
        disk::{load, write, write_solution},
        search::Solver,
    };

    #[test]
    fn test() {
        // let instances = vec![
        //     1, 3, 5, 7, 9, 11, 13, 15, 21, 23, 25, 31, 35, 41, 47, 97, 113, 115, 137,
        // ];
        // let instances = vec![17, 27, 29, 33, 39];
        // let solution = vec![236, 432, 509, 672, 665];
        let instances = vec![19];
        let solution = vec![298];
        for (instance, sol) in instances.into_iter().zip(solution) {
            let time = Instant::now();
            let file_name = format!("../exact/exact{:03}.gr", instance);
            let graph = load(File::open(&file_name).unwrap()).unwrap();
            // critical(&mut graph);
            let mut solver = Solver::new(graph);

            if cfg!(feature = "perfect-upper") {
                solver.upper = sol
            }

            solver.search_components();
            print!("({}, {})", instance, time.elapsed().as_millis());
            io::stdout().flush().unwrap();
            assert_eq!(solver.upper, sol);

            if cfg!(any(feature = "perfect-upper", feature = "branch-comp")) {
                continue;
            }

            let out_file = format!("../exact/solution{:03}.s", instance);
            let out_file2 = format!("../exact/solution{:03}.gr", instance);
            write(
                &solver.graph,
                &solver.best,
                File::create(&out_file2).unwrap(),
            )
            .unwrap();

            let count2 = write_solution(
                &solver.graph,
                &solver.best,
                File::create(&out_file).unwrap(),
            )
            .unwrap();
            assert_eq!(solver.upper, count2);

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

    #[test]
    fn lower_bound() {
        // let mut bounds = Vec::new();
        for instance in (1..200).step_by(2) {
            let file_name = format!("../exact/exact{:03}.gr", instance);
            let graph = load(File::open(file_name).unwrap()).unwrap();
            let solver = Solver::new(graph);
            // critical(&mut graph);
            println!("{:03} {}", instance, solver.packing.lower);
            // let actual = search_components(&mut graph, i32::MAX, &mut Graph::new(0));
            // println!("{:.1}%", 100. * lower as f32 / actual as f32);
            // bounds.push((lower, actual));
        }
        // let percentage = bounds
        //     .iter()
        //     .map(|(l, a)| *l as f32 / *a as f32)
        //     .sum::<f32>()
        //     / bounds.len() as f32;
        // println!("{:.1}%", 100. * percentage)
    }

    #[test]
    fn edge_count() {
        let graph = load(File::open("../exact/exact025.gr").unwrap()).unwrap();
        for vertex in graph.active.clone() {
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
