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
        for vertex1 in self.clusters.clone() {
            for vertex2 in self.clusters.clone() {
                if self[vertex1][vertex2].weight < 0
                    && self.two_edges(vertex1, vertex2, 0).count() <= 1
                {
                    self[vertex1][vertex2] = Edge::none()
                }
            }
        }
    }

    pub fn edge_count(&self) -> i32 {
        let mut total = 0;
        for (i1, v1) in self.clusters(0) {
            for (_, v2) in self.clusters(i1) {
                total += (self[v1][v2].weight > 0) as i32
            }
        }
        total
    }
}

pub fn finish_solve(input: &Graph, output: &mut Graph) {
    let out = unsafe { &*(output as *const Graph) };
    for (i1, v1) in out.clusters(0) {
        for (i2, v2) in out.positive(v1, i1) {
            for pair in out.conflict_edges(v1, v2, i2) {
                let edge_weight = output[v1][v2].weight;
                if edge_weight <= pair.edge1.weight.abs() && edge_weight <= pair.edge2.weight.abs()
                {
                    output.cut(v1, v2);
                } else if pair.edge1.weight > 0 {
                    if pair.edge1.weight <= -pair.edge2.weight {
                        output.cut(v1, pair.to);
                    }
                } else if pair.edge2.weight <= -pair.edge1.weight {
                    output.cut(v2, pair.to);
                }
            }
        }
    }

    for mut v1 in output.clusters.clone() {
        if output[v1].merged.is_some() {
            continue;
        }
        for (_, v2) in output.positive(v1, 0).collect::<Vec<_>>() {
            v1 = output.merge(v1, v2).0;
        }
    }
}

pub fn write_solution<F: Write>(input: &Graph, output: &mut Graph, file: F) -> io::Result<i32> {
    finish_solve(input, output);
    let mut writer = BufWriter::new(file);

    let mut count = 0;
    for (i1, v1) in input.clusters(0) {
        for (_, v2) in input.clusters(i1) {
            let edge = input[v1][v2].weight > 0;
            if edge != (output.root(v1) == output.root(v2)) {
                writeln!(&mut writer, "{} {}", v1 + 1, v2 + 1)?;
                count += 1;
            }
        }
    }
    Ok(count)
}

pub fn write<F: Write>(input: &Graph, output: &mut Graph, file: F) -> io::Result<()> {
    finish_solve(input, output);
    let mut writer = BufWriter::new(file);
    // writeln!(
    //     &mut writer,
    //     "p cep {} {}",
    //     input.clusters.len(),
    //     output.edge_count()
    // )?;

    for (i1, v1) in input.clusters(0) {
        for (_, v2) in input.clusters(i1) {
            if output.root(v1) == output.root(v2) {
                writeln!(&mut writer, "{} {}", v1 + 1, v2 + 1)?;
            }
        }
    }
    Ok(())
}
