use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub struct Graph(Vec<Vertex>);

#[derive(Clone)]
pub struct Vertex {
    size: u32,
    edges: Vec<Edge>,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            size: 1,
            edges: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct Edge {
    number: u32,
    vertex: u32,
}

impl Graph {
    pub fn load(file: File) -> io::Result<Self> {
        let mut reader = BufReader::new(file);
        let v = loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            let mut words = line.split(' ');
            match words.next() {
                Some("c") => continue,
                Some("p") => {
                    assert_eq!(words.next(), Some("cep"));
                    break words.next().unwrap().parse::<usize>().unwrap();
                }
                _ => return Err(io::ErrorKind::InvalidInput.into()),
            }
        };
        let mut vertices = vec![Vertex::default(); v];
        for line in reader.lines() {
            let line = line?;
            let mut words = line.split(' ');
            match words.next() {
                Some("c") => continue,
                Some(word) => {
                    let v1 = word.parse::<usize>().unwrap() - 1;
                    let v2 = words.next().unwrap().parse::<usize>().unwrap() - 1;
                    vertices[v1].edges.push(Edge {
                        number: 1,
                        vertex: v2 as u32,
                    });
                    vertices[v2].edges.push(Edge {
                        number: 1,
                        vertex: v1 as u32,
                    });
                }
                None => return Err(io::ErrorKind::InvalidInput.into()),
            }
        }
        Ok(Self(vertices))
    }
}
