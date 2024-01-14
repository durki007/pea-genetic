use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct MatrixGraph {
    pub(crate) size: usize,
    adj_matrix: Vec<Vec<isize>>,
}

impl Graph for MatrixGraph {
    fn new(size: usize) -> MatrixGraph {
        let mut gr = MatrixGraph {
            size,
            adj_matrix: Vec::with_capacity(size),
        };
        for _ in 0..size {
            gr.adj_matrix.push(vec![0; size.try_into().unwrap()]);
        };
        gr
    }
    fn get_weight(&self, from: usize, to: usize) -> isize {
        self.adj_matrix[from][to]
    }

    fn set_adj_matrix(&mut self, adj_matrix: Vec<Vec<isize>>) {
        self.adj_matrix = adj_matrix;
    }
    fn calculate_path_length(&self, path: &Vec<usize>) -> isize {
        let mut length: isize = 0;
        for i in 0..path.len() - 1 {
            length += self.get_weight(path[i], path[i + 1]);
        }
        length += self.get_weight(path[path.len() - 1], path[0]);
        length
    }
}

impl Display for MatrixGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(&format!("MatrixGraph(size: {})\n", self.size));
        for i in 0..self.size {
            for j in 0..self.size {
                s.push_str(&format!("{:width$}", self.adj_matrix[i][j], width = 4));
            }
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}

pub trait Graph {
    fn new(size: usize) -> Self;
    fn get_weight(&self, from: usize, to: usize) -> isize;
    fn set_adj_matrix(&mut self, adj_matrix: Vec<Vec<isize>>);
    fn calculate_path_length(&self, path: &Vec<usize>) -> isize;

}