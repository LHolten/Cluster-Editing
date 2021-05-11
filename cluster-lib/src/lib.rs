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
    fn test001() {
        let mut graph = load(File::open("../exact/exact001.gr").unwrap()).unwrap();
        assert_eq!(search_graph(&mut graph, u32::MAX), 3);
    }
}
