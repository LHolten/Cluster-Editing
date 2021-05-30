use std::{
    io::{self, BufRead, BufReader, BufWriter, Read, Write},
    usize,
};

use crate::graph::{Edge, Graph};

pub fn load<F: Read>(file: F) -> io::Result<Graph> {
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

    let mut graph: Graph = Graph::new(v);

    for line in reader.lines() {
        let line = line?;
        let mut words = line.split(' ');
        match words.next() {
            Some("c") => continue,
            Some(word) => {
                let v1 = word.parse::<usize>().unwrap() - 1;
                let v2 = words.next().unwrap().parse::<usize>().unwrap() - 1;
                graph[v1][v2].weight = 1;
                graph[v2][v1].weight = 1;
            }
            None => return Err(io::ErrorKind::InvalidInput.into()),
        }
    }

    graph.add_indirect_edges();

    Ok(graph)
}

impl Graph {
    fn add_indirect_edges(&mut self) {
        for vertex1 in self.clusters.iter().collect::<Vec<_>>() {
            for vertex2 in self.clusters.iter().collect::<Vec<_>>() {
                if self[vertex1][vertex2].weight < 0
                    && self.two_edges(vertex1, vertex2).count() <= 1
                {
                    self[vertex1][vertex2] = Edge::none()
                }
            }
        }
    }

    // fn vertex_size(&self, index: VertexIndex) -> u32 {
    //     let edges = self.edges(index).positive().collect::<Vec<_>>();
    //     if edges.is_empty() {
    //         0
    //     } else if edges.len() == 1 {
    //         edges[0].weight as u32
    //     } else {
    //         1
    //     }
    // }

    // fn size(&self) -> u32 {
    //     self.clusters().map(|v| self.vertex_size(v)).sum()
    // }

    // fn edge_count(&self) -> u32 {
    //     self.clusters()
    //         .map(|v| self.edges(v).positive().map(|e| e.weight).sum::<i32>())
    //         .sum::<i32>() as u32
    //         / 2
    // }
}

// pub fn write(graph: &mut Graph, file: File) -> io::Result<()> {
//     let mut writer = BufWriter::new(file);
//     writeln!(&mut writer, "p cep {} {}", graph.size(), graph.edge_count())?;

//     let mut new_index = vec![1];
//     for vertex in 0..graph.vertices.len() as u32 {
//         let vertex = VertexIndex(vertex);
//         if graph[vertex].merged.is_some() {
//             new_index.push(*new_index.last().unwrap());
//             continue;
//         }
//         let size = graph.vertex_size(vertex);
//         new_index.push(new_index.last().unwrap() + size);

//         if size == 1 {
//             for edge in graph.edges(vertex).positive() {
//                 if vertex < edge.to {
//                     break;
//                 }
//                 writeln!(
//                     &mut writer,
//                     "{} {}",
//                     new_index[vertex.0 as usize], new_index[edge.to.0 as usize]
//                 )?
//             }
//         }
//         if size > 1 {
//             for from in new_index[vertex.0 as usize]..(new_index[vertex.0 as usize] + size) {
//                 for from2 in new_index[vertex.0 as usize]..from {
//                     writeln!(&mut writer, "{} {}", from, from2)?
//                 }

//                 for edge in graph.edges(vertex).positive() {
//                     if vertex < edge.to {
//                         break;
//                     }
//                     writeln!(&mut writer, "{} {}", from, new_index[edge.to.0 as usize])?
//                 }
//             }
//         }
//     }

//     Ok(())
// }

pub fn write_solution<F: Write>(input: &Graph, output: &mut Graph, file: F) -> io::Result<()> {
    for mut vertex in output.clusters.iter().collect::<Vec<_>>() {
        if output[vertex].merged.is_some() {
            continue;
        }
        for vertex2 in output.positive(vertex).collect::<Vec<_>>() {
            vertex = output.merge(vertex, vertex2).0;
        }
    }
    let mut writer = BufWriter::new(file);

    for vertex1 in input.clusters.iter() {
        for vertex2 in input.clusters.iter() {
            if vertex2 >= vertex1 {
                break;
            }

            let edge = input[vertex1][vertex2].weight > 0;
            if edge != (output.root(vertex1) == output.root(vertex2)) {
                writeln!(&mut writer, "{} {}", vertex1 + 1, vertex2 + 1)?
            }
        }
    }
    Ok(())
}
