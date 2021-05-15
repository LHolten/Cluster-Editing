mod branch;
mod critical;
pub mod disk;
mod graph;
pub mod kernel;
mod merge;
mod packing;
pub mod search;

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::{
        critical::critical,
        disk::{load, write},
        kernel::{kernel2, kernelize},
        search::search_graph,
    };

    #[test]
    fn test() {
        for instance in (1..50).step_by(2) {
            let file_name = format!("../exact/exact{:03}.gr", instance);
            let mut graph = load(File::open(file_name).unwrap()).unwrap();
            critical(&mut graph);
            println!("{}", search_graph(&mut graph, u32::MAX));
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
