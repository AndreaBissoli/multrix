struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(data: Vec<Vec<f64>>) -> Matrix {
        let rows = data.len();
        let cols = data[0].len();
        Matrix { data, rows, cols }
    }

    fn identity(rows: usize, cols: usize) -> Matrix {
        let mut data: Vec<Vec<f64>> = vec![vec![0.0; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                data[i][j] = match (i,j) {
                    _ if i == j => 1.0,
                    _ => 0.0,
                };
            }
        }
        Matrix { data, rows, cols }
    }
    fn print(&self) {
        for i in &self.data {
            for element in i {
                print!("{}", element);
            }
            println!();
        }
    }
}

fn main() {
    let id = Matrix::identity(4, 3);
    let m = Matrix::new(vec![vec![2.0, 3.0, 4.0], vec![1.0, 2.0, 3.0]]);
    id.print();
    m.print();
}
