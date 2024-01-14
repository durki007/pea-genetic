use std::path::{PathBuf};
use std::fs;
use crate::graph::{Graph, MatrixGraph};

pub fn parse_graph(filename: &PathBuf) -> std::io::Result<MatrixGraph> {
    let str = fs::read_to_string(filename)?;
    let vec: Vec<&str> = str.
        split(|c: char| c.is_whitespace())
        .filter(|s| !s.is_empty())
        .collect();
    // Find dimension
    let mut dim: usize = 0;
    for i in 0..vec.len() {
        if vec[i] == "DIMENSION:" {
            dim = vec[i + 1].parse::<usize>().unwrap();
            break;
        }
    }
    // Find matrix start
    let mut start = 0;
    for i in 0..vec.len() {
        if vec[i] == "EDGE_WEIGHT_SECTION" {
            start = i + 1;
            break;
        }
    }
    // Parse adj matrix
    let mut adj_matrix = Vec::with_capacity(dim);
    for i in 0..dim {
        adj_matrix.push(Vec::with_capacity(dim));
        for j in 0..dim {
            if i == j {
                adj_matrix[i].push(0);
            } else {
                adj_matrix[i].push(vec[start + i * dim + j].parse::<isize>().unwrap());
            }
        }
    }
    if dim == 0 {
        return Result::Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Cannot parse graph dimension"));
    }
    let mut gr = MatrixGraph::new(dim);
    gr.set_adj_matrix(adj_matrix);
    Result::Ok(gr)
}