use std::{
    io::{self, BufRead, BufReader, BufWriter, Read, Write},
    usize,
};

use crate::graph::{AllFrom, Graph};

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
    for i in 0..v * 2 {
        graph[[i, i]].weight = i32::MAX;
    }

    for line in reader.lines() {
        let line = line?;
        let mut words = line.split(' ');
        match words.next() {
            Some("c") => continue,
            Some(word) => {
                let v1 = word.parse::<usize>().unwrap() - 1;
                let v2 = words.next().unwrap().parse::<usize>().unwrap() - 1;
                graph[[v1, v2]].weight = 1;
            }
            None => return Err(io::ErrorKind::InvalidInput.into()),
        }
    }

    // graph.add_indirect_edges();

    Ok(graph)
}

impl Graph {
    pub fn edge_count(&self) -> u32 {
        let mut total = 0;
        for (i1, v1) in self.active.all(0) {
            for (_, v2) in self.active.all(i1) {
                total += (self[[v1, v2]].weight > 0) as u32
            }
        }
        total
    }

    pub fn check_easy(&self) {
        for (i1, v1) in self.active.all(0) {
            for (_, v2) in self.positive(v1, i1) {
                let mut num = 0;
                for _ in self.conflict_edges(v1, v2, 0) {
                    num += 1;
                }
                assert!(num == 0);
            }
        }
    }
}

pub fn finish_solve(output: &mut Graph) {
    // let out = unsafe { &*(output as *const Graph) };
    // for (i1, v1) in out.active.all(0) {
    //     for (i2, v2) in out.positive(v1, i1) {
    //         let edge_weight = output[[v1, v2]].weight;
    //         for (_, v3) in out.conflict_edges(v1, v2, i2) {
    //             let (edge1, edge2) = (output[[v1, v3]], output[[v2, v3]]);
    //             if edge_weight <= edge1.weight.abs() && edge_weight <= edge2.weight.abs() {
    //                 output.cut(v1, v2);
    //             } else if edge1.weight > 0 {
    //                 if edge1.weight <= -edge2.weight {
    //                     output.cut(v1, v3);
    //                 }
    //             } else if edge2.weight <= -edge1.weight {
    //                 output.cut(v2, v3);
    //             }
    //         }
    //     }
    // }

    for mut v1 in output.active.clone() {
        if output.vertex_merged[v1].is_some() {
            continue;
        }
        for (_, v2) in output.positive(v1, 0).collect::<Vec<_>>() {
            v1 = output.merge(v1, v2).0;
            output.vertex_merged[v1] = None
        }
    }
}

pub fn write_solution<F: Write>(input: &Graph, output: &Graph, file: F) -> io::Result<u32> {
    let mut writer = BufWriter::new(file);

    let mut count = 0;
    for (i1, v1) in input.active.all(0) {
        for (_, v2) in input.active.all(i1) {
            let edge = input[[v1, v2]].weight > 0;
            if edge != (output.root(v1) == output.root(v2)) {
                writeln!(&mut writer, "{} {}", v1 + 1, v2 + 1)?;
                count += 1;
            }
        }
    }
    Ok(count)
}

pub fn write<F: Write>(input: &Graph, output: &Graph, file: F) -> io::Result<()> {
    let mut writer = BufWriter::new(file);
    // writeln!(
    //     &mut writer,
    //     "p cep {} {}",
    //     input.clusters.len(),
    //     output.edge_count()
    // )?;

    for (i1, v1) in input.active.all(0) {
        for (_, v2) in input.active.all(i1) {
            if output.root(v1) == output.root(v2) {
                writeln!(&mut writer, "{} {}", v1 + 1, v2 + 1)?;
            }
        }
    }
    Ok(())
}
