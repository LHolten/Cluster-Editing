use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::graph::{Edge, Graph};

pub fn load(file: File) -> io::Result<Graph> {
    let mut reader = BufReader::new(file);
    let v = loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let mut words = line.split(' ');
        match words.next() {
            Some("c") => continue,
            Some("p") => {
                assert_eq!(words.next(), Some("cep"));
                break words.next().unwrap().parse::<u32>().unwrap();
            }
            _ => return Err(io::ErrorKind::InvalidInput.into()),
        }
    };

    let mut graph: Graph = Graph::new(v);

    for line in reader.lines() {
        let line = line?;
        let mut words = line.split(' ');
        match words.next() {
            Some("c") => continue,
            Some(word) => {
                let v1 = word.parse::<u32>().unwrap() - 1;
                let v2 = words.next().unwrap().parse::<u32>().unwrap() - 1;
                graph[v1].edges.push(Edge::new(v2));
                graph[v2].edges.push(Edge::new(v1));
            }
            None => return Err(io::ErrorKind::InvalidInput.into()),
        }
    }
    Ok(graph)
}
