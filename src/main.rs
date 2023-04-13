use std::fs;
use std::fmt;
use std::fmt::Formatter;
use std::ptr::write;
use rand::Rng;
use rayon::prelude::*;

struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(data: Vec<f64>, rows: usize, cols: usize) -> Matrix {
        Matrix { data, rows, cols }
    }

    fn identity(rows: usize, cols: usize) -> Matrix {
        let mut data: Vec<f64> = vec![0.0; cols*rows];

        for i in 0..rows {
            for j in 0..cols {
                data[i*cols + j] = {
                    if i == j { 1.0 } else { 0.0 }
                };
            }
        }
        Matrix { data, rows, cols }
    }

    fn new_rand(rows: usize, cols: usize) -> Matrix {
        let mut data: Vec<f64> = vec![0.0; cols*rows];

        let mut rng = rand::thread_rng();
        for i in 0..rows {
            for j in 0..cols {
                data[i*cols + j] = rng.gen_range(0..10) as f64;
            }
        }
        Matrix { data, rows, cols }
    }

    fn from_file(filename: &str) -> Matrix {
        let contents = match fs::read_to_string(filename) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{} failed to read from file '{}': {:?}",
                          "Error:", filename, e);
                std::process::exit(1);
            }
        };
        let mut data = Vec::new();
        let rows = contents.lines().count();
        let mut cols = 0;
        for line in contents.lines() {
            for num_str in line.split(',') {
                data.push(num_str.parse().unwrap());
            }
            if cols == 0 {cols = line.split(',').count();}
        }
        Matrix { data, rows, cols }
    }

    fn print(&self) {
        for (i, element) in self.data.iter().enumerate() {
            print!("{},", element);
            if (i+1) % self.cols == 0 {
                println!();
            }
        }
    }

    fn write_to_file(&self, filename: &str) {
        let mut matrix_str = String::with_capacity(self.rows * self.cols * 2);
        for (i, element) in self.data.iter().enumerate() {
            matrix_str.push_str(&format!("{},", element));
            if (i+1) % self.cols == 0 {
                matrix_str.push('\n');
            }
        }


        match fs::write(filename, matrix_str) {
            Ok(_) => {}
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
        let mut data = vec![0.0; cols*rows];
        for i in 0..cols*rows {
            let mut c= 0.0;
            let row = i / cols;
            let col = i % cols;
            for k in 0..self.cols {
                c += self.data[row*self.cols + k] * other.data[k*other.cols + col];
            }
            data[i] = c;
        }
        Ok(Matrix { data, rows, cols })
    }

    fn parallel_product(&self, other: &Matrix) -> Result<Matrix, &str> {
        if self.cols != other.rows {
            return Err("Matrices cannot be multiplied");
        }

        let rows = self.rows;
        let cols = other.cols;
        let mut data = vec![0.0; cols*rows];

        data.par_iter_mut()
            .enumerate()
            .for_each(|(i, c)| {
                let row = i / cols;
                let col = i % cols;
                for k in 0..self.cols {
                    *c += self.data[row*self.cols + k] * other.data[k*other.cols + col];
                }
            });

        Ok(Matrix { data, rows, cols })
    }
}

fn main() {
    /*let id = Matrix::identity(4, 3);
    let a = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
    let b = Matrix::new(vec![2.0, 3.0, 4.0, 5.0], 2, 2);
    a.parallel_product(&b).unwrap().print();
    id.print();
    let a = Matrix::from_file("input.txt");
    let b = Matrix::from_file("input2.txt");
    a.parallel_product(&b).unwrap().print();*/
    println!("Generating a");
    let a = Matrix::new_rand(10000, 1000);
    println!("Writing a to file");
    a.write_to_file("a.txt");
    println!("Generating b");
    let b = Matrix::new_rand(1000, 10000);
    println!("Writing b to file");
    b.write_to_file("b.txt");
    println!("Multiplying a and b");
    a.parallel_product(&b).unwrap().write_to_file("output.txt");
}
