pub mod multrix {
    use rand::Rng;
    use rayon::prelude::*;
    use std::fs;

    pub struct Matrix {
        pub data: Vec<f64>,
        pub rows: usize,
        pub cols: usize,
    }

    impl Matrix {
        pub fn new(data: Vec<f64>, rows: usize, cols: usize) -> Matrix {
            Matrix { data, rows, cols }
        }

        pub fn identity(rows: usize, cols: usize) -> Matrix {
            let mut data: Vec<f64> = vec![0.0; cols * rows];

            for i in 0..rows {
                for j in 0..cols {
                    data[i * cols + j] = {
                        if i == j {
                            1.0
                        } else {
                            0.0
                        }
                    };
                }
            }
            Matrix { data, rows, cols }
        }

        pub fn new_rand(rows: usize, cols: usize) -> Matrix {
            let mut data: Vec<f64> = vec![0.0; cols * rows];

            let mut rng = rand::thread_rng();
            for i in 0..rows {
                for j in 0..cols {
                    data[i * cols + j] = rng.gen_range(0..10) as f64;
                }
            }
            Matrix { data, rows, cols }
        }

        pub fn from_file(filename: &str) -> Matrix {
            let contents = match fs::read_to_string(filename) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!(
                        "{} failed to read from file '{}': {:?}",
                        "Error:", filename, e
                    );
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
                if cols == 0 {
                    cols = line.split(',').count();
                }
            }
            Matrix { data, rows, cols }
        }

        pub fn print(&self) {
            for (i, element) in self.data.iter().enumerate() {
                print!("{},", element);
                if (i + 1) % self.cols == 0 {
                    println!();
                }
            }
        }

        pub fn write_to_file(&self, filename: &str) {
            let mut matrix_str = String::with_capacity(self.rows * self.cols * 2);
            for (i, element) in self.data.iter().enumerate() {
                matrix_str.push_str(&format!("{},", element));
                if (i + 1) % self.cols == 0 {
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

        pub fn product(&self, other: &Matrix) -> Result<Matrix, &str> {
            if self.cols != other.rows {
                return Err("Matrices cannot be multiplied");
            }

            let rows = self.rows;
            let cols = other.cols;
            let mut data = vec![0.0; cols * rows];
            for i in 0..cols * rows {
                let mut c = 0.0;
                let row = i / cols;
                let col = i % cols;
                for k in 0..self.cols {
                    c += self.data[row * self.cols + k] * other.data[k * other.cols + col];
                }
                data[i] = c;
            }
            Ok(Matrix { data, rows, cols })
        }

        pub fn parallel_product(&self, other: &Matrix) -> Result<Matrix, &str> {
            if self.cols != other.rows {
                return Err("Matrices cannot be multiplied");
            }

            let rows = self.rows;
            let cols = other.cols;
            let mut data = vec![0.0; cols * rows];

            data.par_iter_mut().enumerate().for_each(|(i, c)| {
                let row = i / cols;
                let col = i % cols;
                for k in 0..self.cols {
                    *c += self.data[row * self.cols + k] * other.data[k * other.cols + col];
                }
            });

            Ok(Matrix { data, rows, cols })
        }
    }
}
