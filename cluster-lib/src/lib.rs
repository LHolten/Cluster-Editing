mod branch;
// mod critical;
pub mod disk;
mod graph;
mod merge;
pub mod search;

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::{disk::load, search::search_graph};

    #[test]
    fn test() {
        for instance in (1..50).step_by(2) {
            let file_name = format!("../exact/exact{:03}.gr", instance);
            let mut graph = load(File::open(file_name).unwrap()).unwrap();
            println!("{}", search_graph(&mut graph, u32::MAX));
        }
    }
}
