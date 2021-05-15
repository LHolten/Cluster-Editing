use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
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

    for vertex in &mut graph.vertices {
        vertex.edges.sort_by_key(|e| e.to)
    }

    for vertex in graph.clusters().collect::<Vec<_>>() {
        for edge in graph.edges(vertex).positive().collect::<Vec<_>>() {
            for edge2 in graph.edges(edge.to).positive().collect::<Vec<_>>() {
                if edge2.to == vertex {
                    continue;
                }
                let pos = graph[vertex]
                    .edges
                    .binary_search_by_key(&edge2.to, |e| e.to);
                if let Err(pos) = pos {
                    graph[vertex].edges.insert(
                        pos,
                        Edge {
                            weight: -1,
                            to: edge2.to,
                            version: u32::MAX,
                        },
                    )
                }
            }
        }
    }

    Ok(graph)
}

impl Graph {
    fn vertex_size(&self, index: u32) -> u32 {
        let edges = self.edges(index).positive().collect::<Vec<_>>();
        if edges.is_empty() {
            0
        } else if edges.len() == 1 {
            edges[0].weight as u32
        } else {
            1
        }
    }

    fn size(&self) -> u32 {
        self.clusters().map(|v| self.vertex_size(v)).sum()
    }

    fn edge_count(&self) -> u32 {
        self.clusters()
            .map(|v| self.edges(v).positive().map(|e| e.weight).sum::<i32>())
            .sum::<i32>() as u32
            / 2
    }
}

pub fn write(graph: &mut Graph, file: File) -> io::Result<()> {
    let mut writer = BufWriter::new(file);
    writeln!(&mut writer, "p cep {} {}", graph.size(), graph.edge_count())?;

    let mut new_index = vec![1];
    for vertex in 0..graph.vertices.len() as u32 {
        if graph[vertex].merged.is_some() {
            new_index.push(*new_index.last().unwrap());
            continue;
        }
        let size = graph.vertex_size(vertex);
        new_index.push(new_index.last().unwrap() + size);

        if size == 1 {
            for edge in graph.edges(vertex).positive() {
                if vertex < edge.to {
                    break;
                }
                writeln!(
                    &mut writer,
                    "{} {}",
                    new_index[vertex as usize], new_index[edge.to as usize]
                )?
            }
        }
        if size > 1 {
            for from in new_index[vertex as usize]..(new_index[vertex as usize] + size) {
                for from2 in new_index[vertex as usize]..from {
                    writeln!(&mut writer, "{} {}", from, from2)?
                }

                for edge in graph.edges(vertex).positive() {
                    if vertex < edge.to {
                        break;
                    }
                    writeln!(&mut writer, "{} {}", from, new_index[edge.to as usize])?
                }
            }
        }
    }

    Ok(())
}
