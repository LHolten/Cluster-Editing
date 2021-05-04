use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::graph::{Edge, Vertex, VertexUnification};

pub fn load(file: File) -> io::Result<VertexUnification> {
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
    let mut vertices = VertexUnification::new();
    vertices.reserve(v);
    for _ in 0..v {
        vertices.new_key(Default::default());
    }

    for line in reader.lines() {
        let line = line?;
        let mut words = line.split(' ');
        match words.next() {
            Some("c") => continue,
            Some(word) => {
                let v1 = word.parse::<u32>().unwrap() - 1;
                let v2 = words.next().unwrap().parse::<u32>().unwrap() - 1;
                vertices.union_value(
                    v1,
                    Vertex {
                        size: 0,
                        edges: vec![Edge {
                            number: 1,
                            index: v2,
                        }],
                    },
                );
                vertices.union_value(
                    v2,
                    Vertex {
                        size: 0,
                        edges: vec![Edge {
                            number: 1,
                            index: v1,
                        }],
                    },
                );
            }
            None => return Err(io::ErrorKind::InvalidInput.into()),
        }
    }
    Ok(vertices)
}
