use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::graph::Graph;

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
                break words.next().unwrap().parse::<usize>().unwrap();
            }
            _ => return Err(io::ErrorKind::InvalidInput.into()),
        }
    };

    let mut problem: Vec<Vec<u32>> = vec![Default::default(); v];

    for line in reader.lines() {
        let line = line?;
        let mut words = line.split(' ');
        match words.next() {
            Some("c") => continue,
            Some(word) => {
                let v1 = word.parse::<u32>().unwrap() - 1;
                let v2 = words.next().unwrap().parse::<u32>().unwrap() - 1;
                problem[v1 as usize].push(v2);
                problem[v2 as usize].push(v1);
            }
            None => return Err(io::ErrorKind::InvalidInput.into()),
        }
    }
    Ok(problem)
}
