mod branch;
// mod critical;
mod disk;
mod graph;
mod merge;
mod search;

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::{disk::load, search::search_graph};

    #[test]
    fn it_works() {
        let mut graph = load(File::open("../exact/exact001.gr").unwrap()).unwrap();
        dbg!(search_graph(&mut graph, u32::MAX));
    }
}
