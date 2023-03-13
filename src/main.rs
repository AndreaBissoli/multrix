use std::fs;
use std::fmt;
use std::fmt::Formatter;
use std::ptr::write;
use rand::Rng;

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
                data[i][j] = {
                    if i == j {1.0}
                    else {0.0}
                };
            }
        }
        Matrix { data, rows, cols }
    }

    fn new_rand(rows: usize, cols: usize) -> Matrix {
        let mut data: Vec<Vec<f64>> = vec![vec![0.0; cols]; rows];

        let mut rng = rand::thread_rng();
        for i in 0..rows {
            for j in 0..cols {
                data[i][j] = rng.gen_range(0..10) as f64;
            }
        }
        Matrix { data, rows, cols }
    }

    fn from_file(filename: &str) -> Matrix {
        let contents = match fs::read_to_string(filename) {
            Ok(v)=> v,
            Err(e) => {
                eprintln!("{} failed to read from file '{}': {:?}",
                    "Error:", filename, e);
                std::process::exit(1);
            }
        };
        let mut data = vec![];
        contents.lines().for_each(|line| {
            let row: Vec<f64> = line.split(',').map(|part| {
                part.parse().unwrap()
            }).collect();
            data.push(row);
        });
        let rows = data.len();
        let cols = data[0].len();

        Matrix { data, rows, cols }
    }

    fn print(&self) {
        for i in &self.data {
            for element in i {
                print!("{} ", element);
            }
            println!();
        }
    }

    fn write_to_file(&self, filename: &str) {
        let mut matrix_str = String::with_capacity(self.rows * self.cols * 2);
        for i in 0..self.rows {
            for element in &self.data[i] {
                matrix_str.push_str(&format!("{},", element));
            }
            matrix_str.push('\n');
        }


        match fs::write(filename, matrix_str) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error: failed to write to file '{}': {:?}", filename, e);
                std::process::exit(1);
            }
        }
    }

    fn product(&self, other: &Matrix) -> Result<Matrix, &str> {
        if self.cols != other.rows {
            return Err("Matrices cannot be multiplied");
        }

        let rows = self.rows;
        let cols = other.cols;
        let mut data = vec![vec![0.0; cols]; rows];

        for i in 0..rows {
            for k in 0..cols {
                let mut c = 0.0;
                for j in 0..self.cols {
                    c += &self.data[i][j] * other.data[j][k];
                }
                data[i][k] = c;
            }
        }
        Ok(Matrix {data, rows, cols})
    }
}


fn main() {
    /*let id = Matrix::identity(4, 3);
    let a = Matrix::new(vec![vec![2.0, 3.0, 4.0], vec![1.0, 2.0, 3.0]]);
    let b = Matrix::new(vec![vec![2.0, 3.0], vec![1.0, 2.0], vec![1.0, 5.0]]);
    a.product(&b).unwrap().print();
    id.print(); */
    /*let a = Matrix::from_file("input.txt");
    let b = Matrix::from_file("input2.txt");
    a.product(&b).unwrap().print();*/
    let a = Matrix::new_rand(10000,1000);
    a.write_to_file("a.txt");
    let b = Matrix::new_rand(1000,10000);
    b.write_to_file("b.txt");
    a.product(&b).unwrap().write_to_file("output.txt");
}
