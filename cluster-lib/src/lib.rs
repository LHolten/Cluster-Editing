mod branch;
mod critical;
mod disk;
mod graph;
mod merge;

extern crate ena;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
